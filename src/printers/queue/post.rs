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
    prelude::*,
    result::QueryResult,
};

use rocket::{
    State,
    response::status::Accepted,
};
use user::guard::UserGuard;
use jobs::{
    *,
    options::JobOptions,
    uid::UID,
};
use printers::queue::task::WorkerTask;

use taskqueue::TaskQueue;

use astacrypto::random_bytes;

#[derive(FromForm)]
pub struct QueuePostQuery
{
    id: u32,
}

#[post("/<device_id>/queue?<query>")]
pub fn print_job(
    user: UserGuard,
    device_id: u16,
    queues: State<HashMap<u16, TaskQueue<HashMap<Vec<u8>, WorkerTask>, ()>>>,
    query: QueuePostQuery,
) -> QueryResult<Option<Accepted<String>>>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Ok(None),
    };

    let result: Option<(u32, Vec<u8>)> = jobs::table
        .select((jobs::id, jobs::options))
        .filter(jobs::id.eq(query.id))
        .filter(jobs::user_id.eq(user.id))
        .first(&user.connection)
        .optional()?;

    let (job_id, job_options): (u32, JobOptions) = match result {
        None => return Ok(None),
        Some((job_id, job_options)) => {
            (job_id, bincode::deserialize(&job_options)
                .expect("deserializing JobOptions"))
        }
    };
    
    let uid = UID::from(random_bytes(20));

    let mut task: HashMap<Vec<u8>, WorkerTask> = HashMap::new();
    task.insert(
        uid.get_bytes(),
        WorkerTask {
           job_id,
           user_id: user.id,
           options: job_options,
        }
    );

    queue.send(&task)
        .expect("sending job to worker queue");

    Ok(Some(Accepted(Some(format!("{:x}", uid)))))
}
