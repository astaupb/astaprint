/// AStAPrint-Worker - accounting.rs
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as
/// published by the Free Software Foundation, either version 3 of the
/// License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this program.  If not, see <https://www.gnu.org/licenses/>.
use bigdecimal::{
    BigDecimal,
    FromPrimitive,
};
use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use legacy::tds::{
    get_credit,
    insert_transaction,
};

use r2d2_redis::RedisConnectionManager;
use redis::lock::Lock;

use snmp::counter::CounterValues;

use journal::lock::JournalLock;

pub struct Accounting
{
    user_id: u32,
    pub lock: Lock,
    baseprice_cent: u32,
    counter: CounterValues,
    credit: BigDecimal,
    value: BigDecimal,
    mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
    redis_pool: Pool<RedisConnectionManager>,
}

impl Accounting
{
    pub fn new(
        user_id: u32,
        counter: CounterValues,
        mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
        redis_pool: Pool<RedisConnectionManager>,
    ) -> Accounting
    {
        let baseprice_cent = 20;

        let mut lock = Lock::new(&format!("{}", user_id), redis_pool.clone());

        if lock.is_grabbed() {
            info!("accounting for {} locked", &user_id);
        }

        lock.grab();

        let _connection = mysql_pool.get().expect("gettting connection from pool");

        let credit = get_credit(user_id);

        let value = BigDecimal::from_u32(0).unwrap();

        Accounting {
            user_id,
            lock,
            baseprice_cent,
            counter,
            credit,
            value,
            mysql_pool,
            redis_pool,
        }
    }

    pub fn not_enough_credit(&self) -> bool
    {
        &self.credit + &self.value
            < BigDecimal::from_u32(self.baseprice_cent).unwrap()
                / BigDecimal::from_u32(100).unwrap()
    }

    /// sets the value which will be accounted given a counter diff as parameter
    /// returns true if there's enough credit for another page
    pub fn set_value(&mut self, counter: CounterValues)
    {
        let value_cent = (counter.print_bw * 5
            + (counter.print_total - counter.print_bw) * 20)
            + (counter.copy_bw * 5 + (counter.copy_total - counter.copy_bw) * 20);

        self.counter = counter;

        debug!("calculated value: {}", value_cent);
        self.value = -(BigDecimal::from_u32(value_cent as u32).unwrap()
            / BigDecimal::from_u32(100).unwrap());
    }

    pub fn finish(self)
    {
        if self.value < BigDecimal::from_u32(0).unwrap() {
            let _connection =
                self.mysql_pool.get().expect("getting mysql connection from pool");

            let credit = &self.credit + &self.value;

            let _lock = JournalLock::from(self.redis_pool.clone());

            use bigdecimal::ToPrimitive;
            insert_transaction(
                self.user_id,
                (self.value * BigDecimal::from(100)).to_i32().unwrap(),
                "Print Job",
                false,
                None,
            );

            info!("inserted new credit for {}: {}", &self.user_id, &credit);
        } else {
            info!("accounting for {} finished without value", self.user_id);
        }
    }
}
