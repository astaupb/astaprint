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
use model::task::worker::WorkerTask;

use rocket::State;

use rocket_contrib::json::Json;

use admin::guard::AdminGuard;
use printers::PrinterQueues;

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

#[get("/printers/<device_id>/queue")]
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
