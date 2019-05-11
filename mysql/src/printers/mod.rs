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
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
    pub description: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}
