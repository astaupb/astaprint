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
pub mod post;
pub mod get;

use pdf::sanitize;

use mysql::jobs::insert::insert_into_jobs;

use model::{
    task::dispatcher::{
        DispatcherTask, DispatcherState,
    },
    job::{
        info::JobInfo,
        options::JobOptions,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DispatcherTaskResponse
{
    pub uid: String,
    pub filename: String,
}

impl<'a> From<&'a DispatcherTask> for DispatcherTaskResponse
{
    fn from(task: &'a DispatcherTask) -> DispatcherTaskResponse
    {
        DispatcherTaskResponse {
            uid: hex::encode(&task.uid[..]),
            filename: task.filename.clone(),
        }
    }
}

pub fn dispatch(task: DispatcherTask, state: DispatcherState)
{
    let hex_uid = hex::encode(&task.uid[..]);
    info!("{} {} started", task.user_id, &hex_uid[..8]);

    let data =
        state.redis_store.get(task.uid).expect("getting file from store");

    let result = sanitize(data);

    let connection =
        state.mysql_pool.get().expect("getting mysql connection from pool");

    let info: Vec<u8> = bincode::serialize(&JobInfo {
        filename: task.filename,
        title: result.title,
        pagecount: result.pagecount,
        colored: result.colored,
        a3: result.a3,
    })
    .expect("serializing JobInfo");

    let options: Vec<u8> = bincode::serialize(&JobOptions::default())
        .expect("serializing JobOptions");

    insert_into_jobs(
        task.user_id,
        info,
        options,
        result.pdf,
        result.pdf_bw,
        result.preview_0,
        result.preview_1,
        result.preview_2,
        result.preview_3,
        &connection,
    )
    .expect("inserting job into table");
}