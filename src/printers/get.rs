// AStAPrint
// Copyright (C) 2018, 2019 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//
// This file is part of AStAPrint
//
// AStAPrint is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use admin::guard::AdminGuard;
use user::guard::UserGuard;
use diesel::prelude::*;
use mysql::printers::select::{
    select_printer_by_device_id,
    select_printers,
};
use printers::{
    queue::get::WorkerTaskResponse,
    response::{
        PrinterResponse,
        UserPrinterResponse,
    },
    PrinterQueues,
};
use rocket::State;
use rocket_contrib::json::Json;
use snmp::tool::*;

#[get("/")]
pub fn get_printers(user: UserGuard) -> QueryResult<Json<Vec<UserPrinterResponse>>>
{
    Ok(Json(select_printers(&user.connection)?.iter().map(UserPrinterResponse::from).collect()))
}

#[get("/<device_id>")]
pub fn get_single_printer(user: UserGuard, device_id: u32) -> QueryResult<Json<UserPrinterResponse>>
{
    Ok(Json(UserPrinterResponse::from(&select_printer_by_device_id(device_id, &user.connection)?)))
}

#[get("/printers")]
pub fn get_printers_as_admin(admin: AdminGuard) -> QueryResult<Json<Vec<PrinterResponse>>>
{
    Ok(Json(select_printers(&admin.connection)?.iter().map(PrinterResponse::from).collect()))
}

#[get("/printers/<device_id>")]
pub fn get_single_printer_as_admin(
    device_id: u32,
    admin: AdminGuard,
    queues: State<PrinterQueues>,
) -> QueryResult<Option<Json<PrinterResponse>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Ok(None),
    };
    let connection: &MysqlConnection = &admin.connection;

    let mut response = PrinterResponse::from(&select_printer_by_device_id(device_id, connection)?);

    let ip = &response.ip;

    let processing = queue.get_processing();
    if !processing.is_empty() {
        response.queue = Some(WorkerTaskResponse::from(&processing[0]));
    }

    if let Ok(counter) = counter(ip) {
        response.counter = Some(counter);
    }

    if let Ok(status) = status(ip) {
        response.status = Some(status);
    }

    Ok(Some(Json(response)))
}
