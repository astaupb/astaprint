/// AStAPrint - Printers
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
pub mod snmp;

pub mod accounting;
pub mod queue;

use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        PooledConnection,
    },
};

use chrono::NaiveDateTime;

pub mod table;
use self::table::*;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printers"]
pub struct Printer
{
    pub id: u16,
    pub hostname: String,
    pub ip: String,
    pub community: String,
    pub mac: String,
    pub device_id: u16,
    pub model_id: u16,
    pub location: String,
    pub description: String,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printer_model"]
pub struct PrinterModel
{
    pub id: u16,
    pub counter_id: u16,
    pub queue_ctl_id: u16,
    pub energy_ctl_id: u16,
    pub description: String,
}


#[derive(Identifiable, Queryable, Debug)]
#[table_name = "printer_counter"]
pub struct Counter
{
    pub id: u16,
    pub total: String,
    pub print_black: String,
    pub print_color: Option<String>,
    pub copy_black: String,
    pub copy_color: Option<String>,
    pub description: String,
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "printer_queue_ctl"]
pub struct QueueCtl
{
    pub id: u16,
    pub oid: String,
    pub cancel: i32,
    pub clear: i32,
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "printer_energy_ctl"]
pub struct EnergyCtl
{
    pub id: u16,
    pub oid: String,
    pub wake: i32,
    pub sleep: i32,
}

pub fn select_device_ids(connection: &PooledConnection<ConnectionManager<MysqlConnection>>) -> Vec<u16>
{
    printers::table.select(printers::device_id).load(connection).expect("fetching device ids")
}
