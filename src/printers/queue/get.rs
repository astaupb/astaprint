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

use model::task::worker::{
    WorkerCommand,
    WorkerTask,
};

use rocket::State;

use rocket_contrib::json::Json;

use admin::guard::AdminGuard;
use user::guard::UserGuard;

use redis::queue::TaskQueueClient;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkerTaskResponse
{
    user_id: u32,
    uid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkerQueueResponse
{
    incoming: Vec<WorkerTaskResponse>,
    processing: Vec<WorkerTaskResponse>,
}

impl<'a> From<&'a WorkerTask> for WorkerTaskResponse
{
    fn from(task: &WorkerTask) -> WorkerTaskResponse
    {
        WorkerTaskResponse {
            user_id: task.user_id,
            uid: hex::encode(&task.uid[..]),
        }
    }
}

#[get("/<device_id>/queue")]
pub fn get_queue(
    _user: UserGuard,
    device_id: u32,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
) -> Option<Json<WorkerQueueResponse>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return None,
    };

    Some(Json(WorkerQueueResponse {
        incoming: queue.get_incoming().iter().map(WorkerTaskResponse::from).collect(),
        processing: queue
            .get_processing()
            .iter()
            .map(|task| WorkerTaskResponse::from(task))
            .collect(),
    }))
}

#[get("/printers/<device_id>/queue")]
pub fn get_queue_as_admin(
    _admin: AdminGuard,
    device_id: u32,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
) -> Option<Json<Option<WorkerTaskResponse>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return None,
    };

    let incoming = queue.get_incoming();
    Some(Json(
        if !incoming.is_empty() {
            Some(WorkerTaskResponse::from(&incoming[1]))
        }
        else {
            None
        },
    ))
}
