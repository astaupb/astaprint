use mysql::printers::Printer;

use snmp::{
    CounterValues,
    StatusValues,
};

use crate::task::worker::WorkerTaskResponse;

/// representation of a printer displayed to an admin
#[derive(Serialize, Debug, Clone)]
pub struct PrinterResponse
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
    pub last_watched: i64,
    pub status: Option<StatusValues>,
    pub counter: Option<CounterValues>,
    pub queue: Option<WorkerTaskResponse>,
}

impl<'a> From<&'a Printer> for PrinterResponse
{
    fn from(printer: &'a Printer) -> PrinterResponse
    {
        PrinterResponse {
            id: printer.id,
            hostname: printer.hostname.clone(),
            ip: printer.ip.clone(),
            community: printer.community.clone(),
            mac: printer.mac.clone(),
            device_id: printer.device_id,
            location: printer.location.clone(),
            has_a3: printer.has_a3,
            coin_operated: printer.coin_operated,
            description: printer.description.clone(),
            watch_toner: printer.watch_toner,
            watch_tray1: printer.watch_tray1,
            watch_tray2: printer.watch_tray2,
            watch_tray3: printer.watch_tray3,
            watch_interval: printer.watch_interval,
            last_watched: printer.last_watched.timestamp(),
            status: None,
            counter: None,
            queue: None,
        }
    }
}

/// representation of a printer displayed to an user
#[derive(Serialize, Debug, Clone)]
pub struct UserPrinterResponse
{
    pub device_id: u32,
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
}

impl<'a> From<&'a Printer> for UserPrinterResponse
{
    fn from(printer: &'a Printer) -> UserPrinterResponse
    {
        UserPrinterResponse {
            device_id: printer.device_id,
            location: printer.location.clone(),
            has_a3: printer.has_a3,
            coin_operated: printer.coin_operated,
        }
    }
}