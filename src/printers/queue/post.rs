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
    response::status::Accepted,
    State,
};

use rocket_contrib::json::Json;

use user::guard::UserGuard;

use redis::queue::{
    CommandClient,
    TaskQueueClient,
};

use sodium::random_bytes;

#[post("/<device_id>/queue/<hex_uid>?<id>")]
pub fn post_to_queue_element(
    user: UserGuard,
    device_id: u32,
    hex_uid: String,
    id: Option<u32>,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
) -> QueryResult<Status>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Ok(Status::new(404, "Queue Not Found")),
    };

    let uid = match hex::decode(&hex_uid) {
        Ok(uid) => uid,
        Err(_) => return Ok(Status::new(400, "Bad Request")),
    };
    debug!("decoded uid: {:x?}", uid);

    let processing = queue.get_processing();
    let incoming = queue.get_incoming();

    if incoming.iter().all(|element| (element.uid != uid.clone()))
        && processing.iter().all(|element| element.uid != uid.clone())
    {
        return Ok(Status::new(404, "Queue Element Not Found"))
    }

    if incoming.iter().all(|element| (element.user_id != user.id))
        && processing.iter().all(|element| (element.user_id != user.id))
    {
        return Ok(Status::new(401, "Unauthorized"))
    }

    let queue = CommandClient::from((queue, &hex_uid[..]));
    if let Some(id) = id {
        queue.send_command(&WorkerCommand::Print(id)).expect("sending print command");
    }
    else {
        queue.send_command(&WorkerCommand::HeartBeat).expect("sending heartbeat command");
    }

    Ok(Status::new(202, "Started Processing"))
}

#[post("/<device_id>/queue?<id>")]
pub fn post_to_queue(
    user: UserGuard,
    device_id: u32,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
    id: Option<u32>,
) -> QueryResult<Option<Accepted<Json<String>>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Ok(None),
    };

    let mut hungup = false;
    let processing = queue.get_processing();
    debug!("processing: {:?}", processing);
    let hex_uid = if processing.len() > 1 && processing[0].user_id == user.id {
        hex::encode(&processing[0].uid)
    }
    else {
        let uid = random_bytes(20);
        let hex_uid = hex::encode(&uid[..]);

        queue
            .send(&WorkerTask {
                uid,
                user_id: user.id,
            })
            .expect("sending job to worker queue");

        hungup = true;

        hex_uid
    };

    if let Some(id) = id {
        let queue = CommandClient::from((queue, &hex_uid[..]));

        queue.send_command(&WorkerCommand::Print(id)).expect("sending print command to worker");

        // send hungup for not locking printer after print job
        if hungup {
            queue.send_command(&WorkerCommand::Hungup).expect("sending hungup command to worker");
        }
    }

    Ok(Some(Accepted(Some(Json(hex_uid)))))
}
