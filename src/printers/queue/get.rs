/// AStAPrint
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;

use printers::queue::task::WorkerTask;

use rocket::{
    State,
};

use rocket_contrib::Json;

use user::guard::UserGuard;

use redis::queue::TaskQueueClient;

#[get("/<device_id>/queue")]
pub fn get_queue(
    _user: UserGuard,
    device_id: u16,
    queues: State<HashMap<u16, TaskQueueClient<WorkerTask>>>,
) -> Option<Json<Vec<WorkerTask>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return None,
    };

    Some(Json(queue.get()))
}
