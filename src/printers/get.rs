use diesel::prelude::*;
use rocket_contrib::json::Json;
use printers::response::PrinterResponse;
use admin::guard::AdminGuard;
use mysql::printers::{
    select::{select_printers, select_printer_by_device_id},
};

#[get("/printers")]
pub fn get_printers(admin: AdminGuard) -> QueryResult<Json<Vec<PrinterResponse>>>
{
    Ok(Json(select_printers(&admin.connection)?
        .iter().map(|x| {
            let connection: &MysqlConnection = &admin.connection;
            PrinterResponse::from((x, connection)) 
        }).collect()
    ))
}

#[get("/printers/<id>")]
pub fn get_single_printer(id: u32, admin: AdminGuard) -> QueryResult<Json<PrinterResponse>>
{
    let connection: &MysqlConnection = &admin.connection;
    Ok(Json(PrinterResponse::from((&select_printer_by_device_id(id, connection)?, connection))))
}
