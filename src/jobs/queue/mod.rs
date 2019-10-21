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

use pdf::sanitize_pdf;

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
    pub keep: Option<bool>,
    pub color: Option<bool>,
    pub a3: Option<bool>,
    pub duplex: Option<u8>,
}

impl<'a> From<&'a DispatcherTask> for DispatcherTaskResponse
{
    fn from(task: &'a DispatcherTask) -> DispatcherTaskResponse
    {
        DispatcherTaskResponse {
            uid: hex::encode(&task.uid[..]),
            filename: task.filename.clone(),
            keep: task.keep,
            color: task.color,
            a3: task.a3,
            duplex: task.duplex,
        }
    }
}

pub fn dispatch(
    task: DispatcherTask,
    state: DispatcherState,
    client: TaskQueueClient<DispatcherTask, ()>,
)
{
    let uid = &hex::encode(&task.uid[..])[.. 8];

    info!("{} {} started", uid, task.user_id);

    let data = if let Ok(data) = state.redis_store.get(task.uid.clone()) {
        data
    }
    else {
        error!("{} getting data from store", uid);
        client.finish(&task).expect("removing task from queue");
        return
    };

    let result = sanitize_pdf(data, uid);

    let connection = if let Ok(connection) = state.mysql_pool.get() {
        connection
    }
    else {
        error!("{} getting mysql connection from pool", uid);
        client.finish(&task).expect("removing task from queue");
        return
    };

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
        landscape: result.landscape,
    })
    .expect("serializing JobInfo");

    let mut options: JobOptions = match select_user_options(task.user_id, &connection) {
        Ok(Some(options)) => bincode::deserialize(&options[..]).expect("deserializing JobOptions"),
        Ok(None) => JobOptions::default(),
        Err(e) => {
            error!("{} selecting user options: {:?}", uid, e);
            client.finish(&task).expect("removing task from queue");
            return
        },
    };

    if let Some(keep) = task.keep {
        options.keep = keep;
    }
    if let Some(a3) = task.a3 {
        options.a3 = a3;
    }
    if let Some(duplex) = task.duplex {
        options.duplex = duplex;
    }
    if let Some(color) = task.color {
        options.color = color;
    }
    if let Some(copies) = task.copies {
        options.copies = copies;
    }

    match insert_into_jobs(
        task.user_id,
        info,
        bincode::serialize(&options).expect("serializing JobOptions"),
        result.pdf,
        result.preview_0,
        result.preview_1,
        result.preview_2,
        result.preview_3,
        &connection,
    ) {
        Ok(_) => {
            info!(
                "{} finished, pagecount: {}, colored: {}, a3: {}",
                uid, result.pagecount, result.colored, result.a3
            );
        },
        Err(e) => {
            error!("{} inserting job: {:?}", uid, e);
        },
    }
    client.finish(&task).expect("removing task from queue");
}
