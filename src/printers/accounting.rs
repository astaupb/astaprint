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

use model::job::Job;

use mysql::{
    journal::select::select_latest_credit_of_user,
    update_credit_after_print,
};

use snmp::CounterValues;

use std::{
    convert::TryInto,
    ops::Drop,
};

pub struct Accounting
{
    user_id: u32,
    printer_id: u32,
    job: Option<Job>,
    credit: i32,
    value: i32,
    expected: u16,
    baseprice: u8,
    basecounter: CounterValues,
    counter: CounterValues,
    mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Accounting
{
    pub fn new(
        user_id: u32,
        printer_id: u32,
        counter: CounterValues,
        mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
    ) -> Accounting
    {
        Accounting {
            user_id,
            printer_id,
            credit: 0,
            job: None,
            value: 0,
            expected: 0,
            baseprice: 5,
            basecounter: counter.clone(),
            counter,
            mysql_pool,
        }
    }

    pub fn credit(&mut self) -> i32
    {
        let connection = self.mysql_pool.get().expect("gettting connection from pool");

        self.credit =
            select_latest_credit_of_user(self.user_id, &connection).expect("selecting credit");

        self.credit
    }

    pub fn value(&self) -> i32 { self.value }

    pub fn pages_left(&self) -> i32 { (self.credit + self.value) / i32::from(self.baseprice) }

    pub fn not_enough_credit(&self) -> bool { self.credit + self.value < i32::from(self.baseprice) }

    pub fn start(&mut self, job: Job, counter: CounterValues)
    {
        self.credit();
        self.expected = job.pages_to_print();
        self.baseprice = if job.options.color {
            20
        }
        else {
            5
        };
        self.job = Some(job);
        self.counter = counter.clone();
        self.basecounter = counter;
    }

    pub fn update(&mut self, counter: Option<CounterValues>) -> bool
    {
        if let Some(counter) = counter {
            if counter.total > self.counter.total {
                let (print_total, print_bw) = (
                    counter.print_total - self.basecounter.print_total,
                    counter.print_bw - self.basecounter.print_bw,
                );
                self.counter = counter;

                self.value = (-(print_bw * 5 + (print_total - print_bw) * 20))
                    .try_into()
                    .expect("value fits in i64");

                if let Ok(connection) = self.mysql_pool.get() {
                    if let Ok(credit) = select_latest_credit_of_user(self.user_id, &connection) {
                        self.credit = credit;
                    }
                }

                info!(
                    "calculated credit for {}: {} + {} = {}",
                    self.user_id,
                    self.credit,
                    self.value,
                    self.credit + self.value
                );

                return true
            }
        }
        false
    }

    pub fn finished(&mut self) -> bool
    {
        self.job.is_some()
            && (self.counter.print_total - self.basecounter.print_total) as u16 == self.expected
            && self.finish()
    }

    pub fn finish(&mut self) -> bool
    {
        if self.value < 0 {
            let connection = self.mysql_pool.get().expect("getting mysql connection from pool");

            let job = self.job.clone().unwrap();
            self.credit = update_credit_after_print(
                self.user_id,
                self.value,
                job.id,
                (self.counter.total - self.basecounter.total).try_into().unwrap(),
                (self.counter.print_total - self.counter.print_bw).try_into().unwrap(),
                job.score(),
                self.printer_id,
                bincode::serialize(&job.options).expect("serializing JobOptions"),
                &connection,
            )
            .expect("updating credit");

            info!("new credit for {}: {}", self.user_id, self.credit);

            self.value = 0;
            self.job = None;
            self.expected = 0;

            return true
        }
        false
    }
}
impl Drop for Accounting
{
    fn drop(&mut self) { self.finish(); }
}
