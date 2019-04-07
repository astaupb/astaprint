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
pub mod get;
pub mod post;

pub mod data;

use pdf::sanitize;

use mysql::{
    jobs::insert::insert_into_jobs,
    user::select::select_user_options,
};

use redis::queue::TaskQueueClient;

use model::{
    job::{
        info::JobInfo,
        options::JobOptions,
    },
    task::dispatcher::{
        DispatcherState,
        DispatcherTask,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DispatcherTaskResponse
{
    pub uid: String,
    pub filename: String,
    pub keep: bool,
}

impl<'a> From<&'a DispatcherTask> for DispatcherTaskResponse
{
    fn from(task: &'a DispatcherTask) -> DispatcherTaskResponse
    {
        DispatcherTaskResponse {
            uid: hex::encode(&task.uid[..]),
            filename: task.filename.clone(),
            keep: task.keep,
        }
    }
}

pub fn dispatch(
    task: DispatcherTask,
    state: DispatcherState,
    client: TaskQueueClient<DispatcherTask, ()>,
)
{
    let hex_uid = hex::encode(&task.uid[..]);
    info!("{} {} started", task.user_id, &hex_uid[.. 8]);

    let data = state.redis_store.get(task.uid.clone()).expect("getting file from store");

    let result = sanitize(data);

    let connection = state.mysql_pool.get().expect("getting mysql connection from pool");

    let filename = if task.filename == "" {
        result.title.clone()
    }
    else {
        task.filename.clone()
    };
    let info: Vec<u8> = bincode::serialize(&JobInfo {
        filename,
        title: result.title,
        pagecount: result.pagecount,
        colored: result.colored,
        a3: result.a3,
    })
    .expect("serializing JobInfo");

    let mut options: JobOptions =
        match select_user_options(task.user_id, &connection).expect("selecting user default options") {
            Some(options) => bincode::deserialize(&options[..]).expect("deserializing JobOptions"),
            None => JobOptions::default()
        };

    options.keep = task.keep;

    insert_into_jobs(
        task.user_id,
        info,
        bincode::serialize(&options).expect("serializing JobOptions"),
        result.pdf,
        result.pdf_bw,
        result.preview_0,
        result.preview_1,
        result.preview_2,
        result.preview_3,
        &connection,
    )
    .expect("inserting job into table");

    client.finish(&task).expect("removing task from queue");
    info!(
        "{} finished, pagecount: {}, colored: {}, a3: {}",
        &hex_uid[.. 8],
        result.pagecount,
        result.colored,
        result.a3
    );
}
