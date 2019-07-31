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
use diesel::prelude::*;
use model::task::worker::{
    WorkerTask,
    WorkerCommand,
};
use mysql::printers::select::{
    select_printer_by_device_id,
    select_printers,
};
use printers::{
    queue::get::WorkerTaskResponse,
    response::PrinterResponse,
};
use redis::queue::TaskQueueClient;
use rocket::State;
use rocket_contrib::json::Json;
use std::collections::HashMap;
#[get("/printers")]
pub fn get_printers(admin: AdminGuard) -> QueryResult<Json<Vec<PrinterResponse>>>
{
    Ok(Json(select_printers(&admin.connection)?.iter().map(PrinterResponse::from).collect()))
}

#[get("/printers/<id>")]
pub fn get_single_printer(
    id: u32,
    admin: AdminGuard,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
) -> QueryResult<Option<Json<PrinterResponse>>>
{
    let queue = match queues.get(&id) {
        Some(queue) => queue,
        None => return Ok(None),
    };
    let connection: &MysqlConnection = &admin.connection;

    let mut response = PrinterResponse::from(select_printer_by_device_id(id, connection)?);

    let processing = queue.get_processing();
    if !processing.is_empty() {
        response.queue = Some(WorkerTaskResponse::from(&processing[0]));
    }
    Ok(Some(Json(response)))
}
