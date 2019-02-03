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

#[post("/<device_id>/queue?<id>")]
pub fn print_job(
    user: UserGuard,
    device_id: u32,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
    id: u32,
) -> QueryResult<Option<Accepted<Json<String>>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Ok(None),
    };

    let uid = random_bytes(20);
    let hex_uid = hex::encode(&uid[..]);

    let task = WorkerTask {
        uid,
        user_id: user.id,
    };

    queue.send(&task).expect("sending job to worker queue");

    let queue = CommandClient::from((queue, &hex_uid[..]));

    queue.send_command(&WorkerCommand::Print(id)).expect("sending print command to worker");

    queue.send_command(&WorkerCommand::Hungup).expect("sending hungup command to worker");

    Ok(Some(Accepted(Some(Json(hex_uid)))))
}
