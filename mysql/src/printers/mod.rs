pub mod select;
pub mod update;
pub mod insert;
pub mod delete;

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
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
    pub description: String,
    pub watch_toner: bool,
    pub watch_tray1: bool,
    pub watch_tray2: bool,
    pub watch_tray3: bool,
    pub watch_interval: u32,
    pub last_watched: NaiveDateTime,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printers"]
pub struct PrinterWatcher
{
    pub id: u32,
    pub name: String,
    pub email: String,
}
