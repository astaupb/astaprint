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

pub fn select_printer_counter(
    connection: &MysqlConnection,
) -> QueryResult<Vec<PrinterCounter>>
{
    printer_counter::table.select(printer_counter::all_columns).load(connection)
}

pub fn select_printer_objects_by_device_id(device_id: u32, connection: &MysqlConnection) -> QueryResult<Option<(PrinterCounter, PrinterControl, PrinterInfo, PrinterStatus)>>
{
    printers::table
        .inner_join(printer_counter::table)
        .inner_join(printer_control::table)
        .inner_join(printer_info::table)
        .inner_join(printer_status::table)
        .select((
            printer_counter::all_columns,
            printer_control::all_columns,
            printer_info::all_columns,
            printer_status::all_columns,
        ))
        .filter(printers::device_id.eq(device_id))
        .first(connection)
        .optional()
}

pub fn select_ip_and_community_by_device_id(device_id: u32, connection: &MysqlConnection) -> QueryResult<Option<(String, String)>>
{
    printers::table
        .select((printers::ip, printers::community))
        .filter(printers::device_id.eq(device_id))
        .first(connection)
        .optional()
}

pub fn select_device_ids(connection: &MysqlConnection) -> QueryResult<Vec<u32>>
{
    printers::table.select(printers::device_id).load(connection)
}

pub fn select_device_id_by_ip(ip: &str, connection: &MysqlConnection) -> QueryResult<u32>
{
    printers::table.select(printers::device_id).filter(printers::ip.eq(ip)).first(connection)
}

pub fn select_all_ips(connection: &MysqlConnection) -> QueryResult<Vec<String>>
{
    printers::table
        .select(printers::ip)
        .load(connection)
}

pub fn select_all_ips_downstairs(connection: &MysqlConnection) -> QueryResult<Vec<String>>
{
    printers::table
        .select(printers::ip)
        .filter(printers::location.ne("BI_2.107"))
        .load(connection)
}