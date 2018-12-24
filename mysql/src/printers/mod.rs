pub mod select;

use crate::schema::*;
use chrono::NaiveDateTime;

joinable!(printers -> printer_counter (counter_id));
joinable!(printers -> printer_control (control_id));
joinable!(printers -> printer_status (status_id));
joinable!(printers -> printer_info (info_id));

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printers"]
pub struct Printer
{
    pub id: u32,
    pub hostname: String,
    pub ip: String,
    pub community: String,
    pub mac: String,
    pub device_id: u32,
    pub counter_id: u32,
    pub control_id: u32,
    pub status_id: u32,
    pub info_id: u32,
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
    pub description: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printer_counter"]
pub struct PrinterCounter
{
    pub id: u32,
    pub total: String,
    pub copy_total: String,
    pub copy_bw: String,
    pub print_total: String,
    pub print_bw: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printer_control"]
pub struct PrinterControl
{
    pub id: u32,
    pub queue: String,
    pub cancel: i32,
    pub clear: i32,
    pub energy: String,
    pub wake: i32,
    pub sleep: i32,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printer_status"]
pub struct PrinterStatus
{
    pub id: u32,
    pub uptime: String,
    pub scan: String,
    pub copy: String,
    pub toner: String,
    pub tray_1: String,
    pub tray_2: String,
    pub tray_3: String,
    pub tray_4: String,
    pub tray_5: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printer_info"]
pub struct PrinterInfo
{
    pub id: u32,
    pub model: String,
    pub hostname: String,
    pub location: String,
    pub mac: String,
    pub crated: NaiveDateTime,
    pub updated: NaiveDateTime,
}

