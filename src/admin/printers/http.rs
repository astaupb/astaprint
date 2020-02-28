use model::task::worker::{
    WorkerCommand,
};

use rocket::{
    State,
    http::Status,
    response::status::Custom,
};

use diesel::prelude::*;

use rocket_contrib::json::Json;

use admin::{
    guard::AdminGuard,
    printers::{
        PrinterResponse,
        WorkerTaskResponse,
        PrinterCreate,
    },
};

use jobs::options::JobOptionsUpdate;

use printers::PrinterQueues;

use printers::update::PrinterUpdate;

use mysql::printers::{
    insert::*,
    update::*,
    select::*,
};

use snmp::tool::{
    counter,
    status,
};

use redis::queue::CommandClient;

#[get("/")]
pub fn get_printers_as_admin(admin: AdminGuard) -> QueryResult<Json<Vec<PrinterResponse>>>
{
    Ok(Json(select_printers(&admin.connection)?.iter().map(PrinterResponse::from).collect()))
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

#[post("/printers", data = "<post>")]
pub fn post_printer(admin: AdminGuard, post: Json<PrinterCreate>) -> QueryResult<Status>
{
    insert_into_printers(PrinterInsert::from(post.into_inner()), &admin.connection)?;

    Ok(Status::new(200, "OK"))
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

#[put("/printers/<id>", data = "<update>")]
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
