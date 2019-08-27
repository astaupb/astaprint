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
use std::collections::HashMap;

use diesel::result::QueryResult;

use model::task::worker::{
    WorkerCommand,
    WorkerTask,
};

use rocket::{
    http::Status,
    response::status::{
        Accepted,
        Custom,
    },
    State,
};

use rocket_contrib::json::Json;

use user::guard::UserGuard;

use redis::queue::{
    CommandClient,
    TaskQueueClient,
};

use sodium::random_bytes;

#[post("/<device_id>/queue?<id>")]
pub fn post_to_queue(
    user: UserGuard,
    device_id: u32,
    id: Option<u32>,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
) -> QueryResult<Result<Accepted<Json<String>>, Custom<()>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Ok(Err(Custom(Status::new(404, "Task Not Found"), ()))),
    };

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

    let queue = CommandClient::from((queue, &hex_uid[..]));
    if let Some(id) = id {
        info!("print job {} command", id);

        queue.send_command(&WorkerCommand::Print(id)).expect("sending print command to worker");

        // send hungup for not locking printer after print job
        if hungup {
            queue.send_command(&WorkerCommand::Hungup).expect("sending hungup command to worker");
        }
    }
    else if !hungup {
        queue.send_command(&WorkerCommand::HeartBeat).expect("sending heartbeat command to worker");
    }

    Ok(Ok(Accepted(Some(Json(hex_uid)))))
}
