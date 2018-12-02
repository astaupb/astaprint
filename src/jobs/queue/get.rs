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
use jobs::task::DispatcherTask;

use rocket::State;

use rocket_contrib::Json;

use user::guard::UserGuard;

use redis::queue::TaskQueueClient;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DispatcherTaskResponse
{
    pub uid: String,
    pub filename: String,
    pub color: bool,
}
impl<'a> From<&'a DispatcherTask> for DispatcherTaskResponse
{
    fn from(task: &'a DispatcherTask) -> DispatcherTaskResponse
    {
        DispatcherTaskResponse {
            uid: hex::encode(&task.uid[..]),
            filename: task.info.filename.clone(),
            color: task.info.color,
        }
    }
}

#[get("/queue")]
pub fn get_dispatcher_queue(
    user: UserGuard,
    queue: State<TaskQueueClient<DispatcherTask>>,
) -> Option<Json<Vec<DispatcherTaskResponse>>>
{
    Some(Json(
        queue
            .get()
            .iter()
            .filter(|element| element.user_id == user.id)
            .map(|element| (*element).clone())
            .map(|task| DispatcherTaskResponse::from(&task))
            .collect(),
    ))
}
