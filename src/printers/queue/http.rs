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
use diesel::result::QueryResult;

use rocket::{
    http::Status,
    response::status::{
        Accepted,
        Custom,
    },
    State,
};

use rocket_contrib::json::Json;

use model::{
    job::options::update::JobOptionsUpdate,
    task::worker::{
        WorkerCommand,
        WorkerTask,
    },
};

use redis::queue::CommandClient;

use sodium::random_bytes;

use crate::{
    user::guard::UserGuard,
    printers::{
        PrinterQueue,
        PrinterQueues,
    },
};

pub fn post_to_queue_handler(
    user: UserGuard,
    id: Option<u32>,
    options: Option<JobOptionsUpdate>,
    queue: PrinterQueue,
) -> QueryResult<Result<Accepted<Json<String>>, Custom<()>>>
{
    let mut hungup = false;
    let processing = queue.get_processing();
    let hex_uid = if !processing.is_empty() && processing[0].user_id == user.id {
        info!("found processing queue element with uid {:x?}", &processing[0].uid[.. 8]);
        hex::encode(&processing[0].uid)
    }
    else {
        if !processing.is_empty() {
            return Ok(Err(Custom(Status::new(423, "Queue Locked"), ())))
        }
        let uid = random_bytes(20);
        let hex_uid = hex::encode(&uid[..]);

        queue
            .send(&WorkerTask {
                uid,
                user_id: user.id,
            })
            .expect("sending job to worker queue");

        info!("created task {} for user {}", &hex_uid[.. 8], user.id);
        hungup = true;
        hex_uid
    };

    let queue = CommandClient::from((&queue, &hex_uid[..]));
    if let Some(id) = id {
        info!("print job {} command", id);

        queue
            .send_command(&WorkerCommand::<Option<JobOptionsUpdate>>::Print((id, options)))
            .expect("sending print command to worker");

        // send hungup for not locking printer after print job
        if hungup {
            queue
                .send_command(&WorkerCommand::<Option<JobOptionsUpdate>>::Hungup)
                .expect("sending hungup command to worker");
        }
    }
    else if !hungup {
        queue
            .send_command(&WorkerCommand::<Option<JobOptionsUpdate>>::HeartBeat)
            .expect("sending heartbeat command to worker");
    }

    Ok(Ok(Accepted(Some(Json(hex_uid)))))
}

#[post("/<device_id>/queue?<id>", data = "<options>")]
pub fn post_to_queue(
    user: UserGuard,
    device_id: u32,
    id: Option<u32>,
    options: Option<Json<JobOptionsUpdate>>,
    queues: State<PrinterQueues>,
) -> QueryResult<Result<Accepted<Json<String>>, Custom<()>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Ok(Err(Custom(Status::new(404, "Printer Not Found"), ()))),
    };
    post_to_queue_handler(user, id, options.map(|o| o.into_inner()), queue.clone())
}

#[delete("/<device_id>/queue")]
pub fn delete_queue(user: UserGuard, device_id: u32, queues: State<PrinterQueues>) -> Custom<()>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Custom(Status::new(404, "Device Not Found"), ()),
    };
    let processing = queue.get_processing();
    if processing.is_empty() {
        return Custom(Status::new(424, "Task Not Found"), ())
    }
    let task = processing[0].clone();
    if task.user_id == user.id {
        let hex_uid = hex::encode(&task.uid[..]);
        debug!("sending cancel to {}", &hex_uid[.. 8]);
        let client = CommandClient::from((queue, &hex_uid[..]));
        client
            .send_command(&WorkerCommand::<Option<JobOptionsUpdate>>::Cancel)
            .expect("sending cancel command");

        Custom(Status::new(205, "Success - Reset Content"), ())
    }
    else {
        Custom(Status::new(403, "Forbidden"), ())
    }
}
