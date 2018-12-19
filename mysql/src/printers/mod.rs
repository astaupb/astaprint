pub mod select;

use crate::schema::*;
use chrono::NaiveDateTime;

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
    pub objects_id: u32,
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
    pub description: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printer_objects"]
pub struct PrinterObjects
{
    pub id: u32,
    pub counter_total: String,
    pub counter_copy_total: String,
    pub counter_copy_bw: String,
    pub counter_print_total: String,
    pub counter_print_bw: String,
    pub queue_ctl: String,
    pub cancel: i32,
    pub clear: i32,
    pub energy_ctl: String,
    pub wake: i32,
    pub sleep: i32,
    pub created: NaiveDateTime,
    pub update: NaiveDateTime,
}
