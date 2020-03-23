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

use diesel::prelude::*;
use rocket::{
    http::Status,
    response::status::Custom,
    State,
};
use rocket_contrib::json::Json;

use redis::queue::CommandClient;

use model::{
    printer::PrinterResponse,
    task::worker::{
        WorkerCommand,
        WorkerTaskResponse,
    },
};

use mysql::printers::{
    delete::*,
    insert::*,
    select::*,
    update::*,
};

use snmp::tool::{
    counter,
    status,
};

use admin::guard::AdminGuard;

use jobs::options::JobOptionsUpdate;

use printers::{
    update::PrinterUpdate,
    PrinterQueues,
};
#[get("/")]
pub fn get_printers_as_admin(admin: AdminGuard) -> QueryResult<Json<Vec<PrinterResponse>>>
{
    Ok(Json(select_printers(&admin.connection)?.iter().map(PrinterResponse::from).collect()))
}

#[post("/printers", data = "<post>")]
pub fn post_printer(admin: AdminGuard, post: Json<PrinterInsert>) -> QueryResult<Status>
{
    insert_into_printers(PrinterInsert::from(post.into_inner()), &admin.connection)?;

    Ok(Status::new(205, "Success - Reset Content"))
}

#[get("/<device_id>")]
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

#[delete("/<id>")]
pub fn delete_printer(admin: AdminGuard, id: u32) -> QueryResult<Status>
{
    delete_printer_by_device_id(id, &admin.connection)?;
    Ok(Status::new(205, "Reset Content"))
}

#[put("/<id>", data = "<update>")]
pub fn put_printer_details(
    admin: AdminGuard,
    id: u32,
    update: Json<PrinterUpdate>,
) -> QueryResult<Status>
{
    let printer = update.into_inner().update(select_printer_by_device_id(id, &admin.connection)?);
    update_printer(printer, &admin.connection)?;
    Ok(Status::new(205, "Reset Content"))
}

#[get("/<device_id>/queue")]
pub fn get_queue_as_admin(
    _admin: AdminGuard,
    device_id: u32,
    queues: State<PrinterQueues>,
) -> Option<Json<Option<WorkerTaskResponse>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return None,
    };

    let processing = queue.get_processing();

    Some(Json(
        if !processing.is_empty() {
            Some(WorkerTaskResponse::from(&processing[0]))
        }
        else {
            None
        },
    ))
}

#[delete("/<device_id>/queue")]
pub fn delete_queue_as_admin(
    admin: AdminGuard,
    device_id: u32,
    queues: State<PrinterQueues>,
) -> Custom<()>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Custom(Status::new(404, "Device Not Found"), ()),
    };
    let processing = queue.get_processing();
    if !processing.is_empty() {
        let client = CommandClient::from((queue, &hex::encode(&processing[0].uid[..])[..]));
        client
            .send_command(&WorkerCommand::<Option<JobOptionsUpdate>>::Cancel)
            .expect("sending cancel command");

        info!("admin {} cleared queue of printer {}", admin.id, device_id);

        Custom(Status::new(205, "Success - Reset Content"), ())
    }
    else {
        Custom(Status::new(424, "Task Not Found"), ())
    }
}
