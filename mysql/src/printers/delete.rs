use crate::{
    schema::*,
};
use diesel::prelude::*;

pub fn delete_printer_by_device_id(id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    diesel::delete(
        printers::table
            .filter(printers::device_id.eq(id))
        )
        .execute(connection)
}
