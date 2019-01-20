use rocket_contrib::json::Json;
use diesel::prelude::*;
use snmp::PrinterInterface;

#[derive(Serialize, Debug, Clone)]
pub struct PrinterStatus
{
    pub toner: i64,
    pub tray_1: i64,
    pub tray_2: i64,
    pub tray_3: i64,
    pub tray_4: i64,
}
