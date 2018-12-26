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

use diesel::{
    result::QueryResult,
};

use jobs::{
    options::JobOptions,
};

use printers::queue::task::WorkerTask;
use rocket::{
    response::status::Accepted,
    State,
};

use rocket_contrib::Json;

use user::guard::UserGuard;

use redis::queue::TaskQueueClient;

use astacrypto::random_bytes;

use mysql::jobs::select::*;

#[derive(FromForm)]
pub struct QueuePostQuery
{
    id: u32,
}

#[post("/<device_id>/queue?<query>")]
pub fn print_job(
    user: UserGuard,
    device_id: u16,
    queues: State<HashMap<u16, TaskQueueClient<WorkerTask>>>,
    query: QueuePostQuery,
) -> QueryResult<Option<Accepted<Json<String>>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Ok(None),
    };

    let job_options: JobOptions =
        match select_job_options(query.id, user.id, &user.connection)? {
        None => return Ok(None),
        Some(job_options) => {
            bincode::deserialize(&job_options).expect("deserializing JobOptions")
        },
    };

    let uid = random_bytes(20);
    let hex_uid = hex::encode(&uid[..]);

    let task = WorkerTask {
        job_id: query.id,
        uid,
        user_id: user.id,
        options: job_options,
    };

    queue.send(&task).expect("sending job to worker queue");

    Ok(Some(Accepted(Some(Json(hex_uid)))))
}
