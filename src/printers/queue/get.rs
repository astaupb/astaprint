/// AStAPrint
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as
/// published by the Free Software Foundation, either version 3 of the
/// License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::collections::HashMap;

use model::task::worker::WorkerTask;

use rocket::State;

use rocket_contrib::json::Json;

use user::guard::UserGuard;

use redis::queue::TaskQueueClient;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum QueueElement
{
    own(WorkerTask),
    foreign(String),
}

#[get("/<device_id>/queue")]
pub fn get_queue(
    user: UserGuard,
    device_id: u16,
    queues: State<HashMap<u16, TaskQueueClient<WorkerTask>>>,
) -> Option<Json<Vec<QueueElement>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return None,
    };

    Some(Json(
        queue
            .get()
            .iter()
            .map(|element| {
                if element.user_id == user.id {
                    QueueElement::own((*element).clone())
                } else {
                    QueueElement::foreign(hex::encode(&element.uid[..]))
                }
            })
            .collect(),
    ))
}
