/// AStAPrint-Common - Accounting.rs
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

use super::lock::Lock;
use snmp::counter::CounterValues;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountWorkerJSON {
    pub account: AccountData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountData {
    pub username: String,
    pub uid: String,
    pub counter: CounterValues,
}

pub struct AccountWork {
    pub data: AccountData,
    pub lock: Lock,
}

impl AccountWork {
    pub fn new(data: AccountData) -> AccountWork {
        let spooldir = env::var("ASTAPRINT_SPOOL_DIR").expect("reading spooldir from environment");
        let lock = Lock::new(&format!("{}/user/{}/accounting", spooldir, &data.username));
        AccountWork { data, lock }
    }
}
