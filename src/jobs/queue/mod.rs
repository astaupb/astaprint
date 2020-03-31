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
//! module containing the dispatcher logic
pub mod http;

pub mod data;

use pdf::sanitize_pdf;

use mysql::{
    jobs::insert::{
        insert_into_jobs,
        JobInsert,
    },
    user::select::select_user_options,
};

use redis::{
    queue::TaskQueueClient,
    store::Store,
};

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

/// helper function for preparing a dispatcher task
pub fn start_dispatch(
    user_id: u32,
    data: Vec<u8>,
    filename: Option<String>,
    preprocess: Option<u8>,
    keep: Option<bool>,
    a3: Option<bool>,
    color: Option<bool>,
    duplex: Option<u8>,
    copies: Option<u16>,
    store: &Store,
    client: &TaskQueueClient<DispatcherTask, ()>,
) -> String
{
    let uid = store.set(data).expect("saving file in store");

    let hex_uid = hex::encode(&uid[..]);

    let filename = if let Some(filename) = filename {
        if filename.len() < 80 {
            filename
        }
        else {
            format!("{}...", &filename[.. 79])
        }
    }
    else {
        String::from("")
    };

    let displayname = Some(filename.clone());

    // default to normal preprocessing
    let preprocess = preprocess.unwrap_or(1);

    let task = DispatcherTask {
        user_id,
        uid,
        filename,
        preprocess,
        keep,
        a3,
        color,
        duplex,
        copies,
        displayname,
    };

    client.send(&task).expect("sending task to queue");

    info!("{} start dispatcher of with uid {} on level {}", user_id, hex_uid, preprocess);

    hex_uid
}

/// this is the main dispatcher logic
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

    let result = sanitize_pdf(data, uid, task.preprocess);

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

    let info: Vec<u8> = JobInfo {
        filename,
        title: result.title,
        pagecount: result.pagecount,
        colored: result.colored,
        a3: result.a3,
        landscape: result.landscape,
    }
    .serialize();

    let mut options: JobOptions = match select_user_options(task.user_id, &connection) {
        Ok(Some(options)) => JobOptions::from(&options[..]),
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
    if let Some(displayname) = task.displayname.clone() {
        options.displayname = displayname;
    }

    match insert_into_jobs(
        JobInsert {
            user_id: task.user_id,
            info,
            options: options.serialize(),
            pdf: result.pdf,
            preview_0: result.preview_0,
            preview_1: result.preview_1,
            preview_2: result.preview_2,
            preview_3: result.preview_3,
        },
        &connection,
    ) {
        Ok(_) => {
            info!(
                "{} finished, pagecount: {}/{}{}{}",
                uid,
                result.colored,
                result.pagecount,
                if result.a3 {
                    ", a3"
                }
                else {
                    ""
                },
                if result.landscape {
                    ", landscape"
                }
                else {
                    ""
                },
            );
        },
        Err(e) => {
            error!("{} inserting job: {:?}", uid, e);
        },
    }
    client.finish(&task).expect("removing task from queue");
}
