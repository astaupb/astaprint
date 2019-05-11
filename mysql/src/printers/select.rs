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

pub fn select_printer_by_device_id(
    device_id: u32,
    connection: &MysqlConnection
) -> QueryResult<Printer>
{
    printers::table
        .select(printers::all_columns)
        .filter(printers::device_id.eq(device_id))
        .first(connection)
}

pub fn select_ip_and_community_by_device_id(device_id: u32, connection: &MysqlConnection) -> QueryResult<Option<(String, String)>>
{
    printers::table
        .select((printers::ip, printers::community))
        .filter(printers::device_id.eq(device_id))
        .first(connection)
        .optional()
}

pub fn select_ip_by_device_id(device_id: u32, connection: &MysqlConnection) -> QueryResult<String>
{
    printers::table
        .select(printers::ip)
        .filter(printers::device_id.eq(device_id))
        .first(connection)
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
