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
use model::task::dispatcher::DispatcherTask;

use jobs::queue::DispatcherTaskResponse;

use rocket::State;

use rocket_contrib::json::Json;

use user::guard::UserGuard;
use admin::guard::AdminGuard;

use redis::queue::TaskQueueClient;

#[get("/queue")]
pub fn get_dispatcher_queue(
    user: UserGuard,
    queue: State<TaskQueueClient<DispatcherTask, ()>>,
) -> Option<Json<Vec<DispatcherTaskResponse>>>
{
    Some(Json(
        queue
            .get_processing()
            .iter()
            .filter(|element| element.user_id == user.id)
            .map(|element| (*element).clone())
            .map(|task| DispatcherTaskResponse::from(&task))
            .collect(),
    ))
}

#[get("/jobs/queue")]
pub fn get_dispatcher_queue_as_admin(
    _admin: AdminGuard,
    queue: State<TaskQueueClient<DispatcherTask, ()>>,
) -> Option<Json<Vec<DispatcherTaskResponse>>>
{
    Some(Json(
        queue
            .get_processing()
            .iter()
            .map(|element| (*element).clone())
            .map(|task| DispatcherTaskResponse::from(&task))
            .collect(),
    ))
}
