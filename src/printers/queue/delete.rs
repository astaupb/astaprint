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

use model::task::worker::{
    WorkerCommand,
    WorkerTask,
};

use rocket::{
    http::Status,
    State,
};

use user::guard::UserGuard;

use redis::queue::{
    CommandClient,
    TaskQueueClient,
};

#[delete("/<device_id>/queue/<hex_uid>")]
pub fn delete_queue(
    user: UserGuard,
    device_id: u32,
    hex_uid: String,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
) -> Status
{
    let uid = match hex::decode(&hex_uid) {
        Ok(uid) => uid,
        Err(_) => return Status::new(400, "Bad Request"),
    };
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Status::new(404, "Device Not Found"),
    };
    let incoming = queue.get_incoming();
    for task in incoming {
        if task.uid == uid && task.user_id == user.id {
            if queue.remove(uid.clone()).expect("removing task") > 0 {
                return Status::new(205, "Success - No Content");
            } else {
                return Status::new(500, "Internal Server Error");
            }
        }
    }
    let processing = queue.get_processing();
    if processing.len() > 0 && processing[0].uid == uid {
        if processing[0].user_id == user.id {
            let client = CommandClient::from((queue, &hex_uid[..]));
            client
                .send_command(&WorkerCommand::Cancel)
                .expect("sending cancel command");

            return Status::new(205, "Success - No Content");
        } else {
            return Status::new(401, "Unauthorized");
        }
    } else {
        return Status::new(404, "Task Not Found");
    }
}

#[delete("/printers/<device_id>/queue/<hex_uid>")]
pub fn delete_queue_as_admin(
    user: UserGuard,
    device_id: u32,
    hex_uid: String,
    queues: State<HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>>>,
) -> Status
{
    let uid = match hex::decode(&hex_uid) {
        Ok(uid) => uid,
        Err(_) => return Status::new(400, "Bad Request"),
    };
    let queue = match queues.get(&device_id) {
        Some(queue) => queue,
        None => return Status::new(404, "Device Not Found"),
    };
    let incoming = queue.get_incoming();
    for task in incoming {
        if task.uid == uid && task.user_id == user.id {
            if queue.remove(uid.clone()).expect("removing task") > 0 {
                return Status::new(205, "Success - No Content");
            } else {
                return Status::new(500, "Internal Server Error");
            }
        }
    }
    let processing = queue.get_processing();
    if processing.len() > 0 && processing[0].uid == uid {
        let client = CommandClient::from((queue, &hex_uid[..]));
        client
            .send_command(&WorkerCommand::Cancel)
            .expect("sending cancel command");

        return Status::new(205, "Success - No Content");
    } else {
        return Status::new(404, "Task Not Found");
    }
}
