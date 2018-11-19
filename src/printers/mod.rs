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

use chrono::NaiveDateTime;

table! {
    printers (id) {
        id -> Unsigned<Smallint>,
        hostname -> Varchar,
        ip -> Varchar,
        community -> Varchar,
        mac -> Varchar,
        device_id -> Unsigned<Smallint>,
        model_id -> Unsigned<Smallint>,
        location -> Varchar,
        description -> Varchar,
        updated -> Timestamp,
    }
}

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

table! {
    printer_model (id) {
        id -> Unsigned<Smallint>,
        counter_id -> Unsigned<Smallint>,
        queue_ctl_id -> Unsigned<Smallint>,
        energy_ctl_id -> Unsigned<Smallint>,
        description -> Varchar,
    }
}

joinable!(printers -> printer_model (model_id));

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

table! {
    printer_counter (id) {
        id -> Unsigned<Smallint>,
        total -> Varchar,
        print_black -> Varchar,
        print_color -> Nullable<Varchar>,
        copy_black -> Varchar,
        copy_color -> Nullable<Varchar>,
        description -> Varchar,
    }
}

joinable!(printer_model -> printer_counter (counter_id));

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

table! {
    printer_queue_ctl (id) {
        id -> Unsigned<Smallint>,
        oid -> Varchar,
        cancel -> Integer,
        clear -> Integer,
    }
}

joinable!(printer_model -> printer_queue_ctl (queue_ctl_id));

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "printer_queue_ctl"]
pub struct QueueCtl
{
    pub id: u16,
    pub oid: String,
    pub cancel: i32,
    pub clear: i32,
}

table! {
    printer_energy_ctl (id) {
        id -> Unsigned<Smallint>,
        oid -> Varchar,
        wake -> Integer,
        sleep -> Integer,
    }
}

joinable!(printer_model -> printer_energy_ctl (energy_ctl_id));

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "printer_energy_ctl"]
pub struct EnergyCtl
{
    pub id: u16,
    pub oid: String,
    pub wake: i32,
    pub sleep: i32,
}

allow_tables_to_appear_in_same_query!(printers, printer_model, printer_counter, printer_energy_ctl, printer_queue_ctl,);
