/// AStAPrint-Worker - accounting.rs
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use bigdecimal::{
    BigDecimal,
    FromPrimitive,
};
use diesel::{
    insert_into,
    prelude::*,
};

use std::env;

use astaprint::{
    database::{
        establish_connection,
        user::schema::*,
    },
    lock::Lock,
};

use super::snmp::counter::CounterValues;

pub struct Accounting
{
    user_id: u32,
    pub lock: Lock,
    baseprice_cent: u32,
    credit: BigDecimal,
    value: BigDecimal,
    connection: MysqlConnection,
}

impl Accounting
{
    pub fn new(user_id: u32, color: bool) -> Accounting
    {
        let userdir = env::var("ASTAPRINT_USER_DIR").expect("reading userdir from environment");

        let lock = Lock::new(&format!("{}/{}/accounting", userdir, user_id));

        if lock.is_grabbed() {
            info!("accounting for {} locked", &user_id);
        }

        let baseprice_cent = if color {
            10
        } else {
            2
        };

        let connection = establish_connection();
        lock.grab();

        let credit: BigDecimal = user::table
            .inner_join(journal::table)
            .select(journal::credit)
            .filter(user::id.eq(journal::user_id))
            .filter(user::id.eq(user_id))
            .order(journal::id.desc())
            .first(&connection)
            .expect("fetching last credit from journal");

        let value = BigDecimal::from_u32(0).unwrap();

        Accounting {
            user_id,
            lock,
            baseprice_cent,
            credit,
            value,
            connection,
        }
    }

    pub fn not_enough_credit(&self) -> bool
    {
        &self.credit + &self.value
            < BigDecimal::from_u32(self.baseprice_cent).unwrap() / BigDecimal::from_u32(100).unwrap()
    }

    /// sets the value which will be accounted given a counter diff as parameter
    /// returns true if there's enough credit for another page
    pub fn set_value(&mut self, counter: &CounterValues)
    {
        let value_cent = counter.print_black * 2
            + counter.print_color.unwrap_or(0) * 10
            + counter.copy_black * 2
            + counter.copy_color.unwrap_or(0) * 10;

        debug!("calculated value: {}", value_cent);
        self.value =
            -(BigDecimal::from_u32(value_cent as u32).unwrap() / BigDecimal::from_u32(100).unwrap());
    }

    pub fn finish(self)
    {
        if self.value < BigDecimal::from_u32(0).unwrap() {
            let credit = &self.credit + &self.value;
            insert_into(journal::table)
                .values((
                    journal::user_id.eq(&self.user_id),
                    journal::value.eq(&self.value),
                    journal::credit.eq(&credit),
                    journal::description.eq("Print Job"),
                ))
                .execute(&self.connection)
                .expect("inserting new journal entry");

            info!("inserted new credit for {}: {}", &self.user_id, &credit);
        } else {
            info!("accounting for {} finished without value", self.user_id);
        }
    }
}
