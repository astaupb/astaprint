use crate::{
    printers::*,
    schema::*,
};
use diesel::prelude::*;

pub fn select_printers(
    connection: &MysqlConnection,
) -> QueryResult<Vec<Printer>>
{
    printers::table.select(printers::all_columns).load(connection)
}

pub fn select_printer_objects(
    connection: &MysqlConnection,
) -> QueryResult<Vec<PrinterObjects>>
{
    printer_objects::table.select(printer_objects::all_columns).load(connection)
}

pub fn select_device_ids(connection: &MysqlConnection) -> QueryResult<Vec<u32>>
{
    printers::table.select(printers::device_id).load(connection)
}

pub fn select_device_id_by_ip(ip: &str, connection: &MysqlConnection) -> QueryResult<u32>
{
    printers::table.select(printers::device_id).filter(printers::ip.eq(ip)).first(connection)
}

