use diesel::prelude::*;
use mysql::printers::Printer;
use snmp::{
    session::SnmpSession,
    status::StatusValues,
    PrinterInterface,
};
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
    pub scan: i64,
    pub copy: i64,
    pub toner: i64,
    pub tray_1: i64,
    pub tray_2: i64,
    pub tray_3: i64,
    pub tray_4: i64,
}

impl<'a> From<(&'a Printer, &'a MysqlConnection)> for PrinterResponse
{
    fn from((printer, connection): (&Printer, &MysqlConnection))
        -> PrinterResponse
    {
        let status = SnmpSession::new(PrinterInterface::from_device_id(
            printer.device_id,
            connection,
        ))
        .get_status()
        .unwrap_or(StatusValues {
            scan: -1,
            copy: -1,
            toner: -1,
            tray_1: -1,
            tray_2: -1,
            tray_3: -1,
            tray_4: -1,
        });

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
            scan: status.scan,
            copy: status.copy,
            toner: status.toner,
            tray_1: status.tray_1,
            tray_2: status.tray_2,
            tray_3: status.tray_3,
            tray_4: status.tray_4,
        }
    }
}
