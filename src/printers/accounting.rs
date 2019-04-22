// AStAPrint
// Copyright (C) 2018, 2019 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//
// This file is part of AStAPrint
//
// AStAPrint is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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

use snmp::CounterValues;

use std::ops::Drop;

pub struct Accounting
{
    user_id: u32,
    baseprice: u32,
    counter: CounterValues,
    credit: i32,
    value: i32,
    mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Accounting
{
    pub fn new(
        user_id: u32,
        counter: CounterValues,
        mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Accounting
    {
        let baseprice = 20;

        let _connection = mysql_pool.get().expect("gettting connection from pool");

        let credit = get_credit(user_id);

        let value = 0;

        Accounting {
            user_id,
            baseprice,
            counter,
            credit,
            value,
            mysql_pool,
        }
    }

    pub fn value(&self) -> i32 { self.value }

    pub fn set_baseprice(
        &mut self,
        baseprice: u32,
    )
    {
        self.baseprice = baseprice
    }

    pub fn bw_pages_left(&self) -> i32 { (self.credit + self.value) / 5 }

    pub fn not_enough_credit(&self) -> bool { &self.credit + &self.value < self.baseprice as i32 }

    /// sets the value which will be accounted given a counter diff as parameter
    /// returns true if there's enough credit for another page
    pub fn set_value(
        &mut self,
        counter: CounterValues,
    )
    {
        self.value = -((counter.print_bw * 5 + (counter.print_total - counter.print_bw) * 20)
            + (counter.copy_bw * 5 + (counter.copy_total - counter.copy_bw) * 20))
            as i32;

        self.counter = counter;

        debug!("calculated value for {}: {}", self.user_id, self.value);
    }
}
impl Drop for Accounting
{
    fn drop(&mut self)
    {
        if self.value < 0 {
            let _connection = self.mysql_pool.get().expect("getting mysql connection from pool");

            let credit = &self.credit + &self.value;

            let total = self.counter.print_total;
            let colored = total - self.counter.print_bw;

            insert_transaction(
                self.user_id,
                self.value,
                &format!("{} Seiten gedruckt, davon {} in Farbe", total, colored),
                false,
                None,
            );

            info!("new credit for {}: {}", &self.user_id, &credit);
        }
        else {
            info!("accounting for {} finished without value", self.user_id);
        }
    }
}
