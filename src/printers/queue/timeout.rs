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

use std::{
    fmt,
    time::{
        Duration,
        SystemTime,
    },
};

#[derive(Clone)]
pub struct TimeOut
{
    pub value: Duration,
    pub begin: SystemTime,
}

impl fmt::Debug for TimeOut
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self.begin.elapsed() {
            Ok(elapsed) => {
                if self.value > elapsed {
                    write!(f, "TimeOut in {} ms", (self.value - elapsed).as_millis(),)
                }
                else {
                    write!(f, "TimeOut: 0")
                }
            },
            Err(e) => write!(f, "{}", e),
        }
    }
}

impl TimeOut
{
    pub fn new(timeout_s: u64) -> TimeOut
    {
        TimeOut {
            value: Duration::from_secs(timeout_s),
            begin: SystemTime::now(),
        }
    }

    pub fn refresh(&mut self) { self.begin = SystemTime::now(); }

    pub fn set_value_in_s(&mut self, timeout_s: u64)
    {
        self.value = Duration::from_secs(timeout_s);
    }

    pub fn check(&mut self) -> bool
    {
        match self.begin.elapsed() {
            Ok(elapsed) => (elapsed > self.value),
            Err(_e) => false,
        }
    }
}
