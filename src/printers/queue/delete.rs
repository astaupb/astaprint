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

use rocket::{
    http::Status,
    response::status::Custom,
    State,
};

use admin::guard::AdminGuard;
use user::guard::UserGuard;

use redis::queue::{
    CommandClient,
    TaskQueueClient,
};

use jobs::options::JobOptionsUpdate;

#[delete("/<device_id>/queue")]
pub fn delete_queue(
    user: UserGuard,
    device_id: u32,
    queues: State<
        HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand<Option<JobOptionsUpdate>>>>,
    >,
) -> Custom<()>
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
        Custom(Status::new(401, "Unauthorized"), ())
    }
}

#[delete("/printers/<device_id>/queue")]
pub fn delete_queue_as_admin(
    admin: AdminGuard,
    device_id: u32,
    queues: State<
        HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand<Option<JobOptionsUpdate>>>>,
    >,
) -> Custom<()>
{
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Custom(Status::new(404, "Device Not Found"), ()),
    };
    let processing = queue.get_processing();
    if !processing.is_empty() {
        let client = CommandClient::from((queue, &hex::encode(&processing[0].uid[..])[..]));
        client
            .send_command(&WorkerCommand::<Option<JobOptionsUpdate>>::Cancel)
            .expect("sending cancel command");

        info!("admin {} cleared queue of printer {}", admin.id, device_id);

        Custom(Status::new(205, "Success - Reset Content"), ())
    }
    else {
        Custom(Status::new(424, "Task Not Found"), ())
    }
}
