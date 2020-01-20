// AStAPrint
// Copyright (C) 2019 AStA der Universität Paderborn
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

use diesel::QueryResult;

use rocket_contrib::json::Json;

use rocket::{
    http::Status,
    State,
};

use redis::share::Share;

use mysql::jobs::{
    insert::*,
    select::*,
};
use user::guard::UserGuard;

use model::{
    job::{
        info::JobInfo,
        options::JobOptions,
    },
    task::dispatcher::DispatcherTask,
};

use redis::{
    queue::TaskQueueClient,
    store::Store,
};

#[post("/<id>?<image>")]
pub fn copy_job(
    user: UserGuard,
    id: u32,
    taskqueue: State<TaskQueueClient<DispatcherTask, ()>>,
    store: State<Store>,
    image: Option<bool>,
) -> QueryResult<Status>
{
    if let Ok(job) = select_full_job_by_id(id, &user.connection) {
        let image = image.unwrap_or(false);
        if image {
            // dispatch again with image option
            let options: JobOptions =
                bincode::deserialize(&job.options).expect("deserializing JobOptions");

            let info: JobInfo = bincode::deserialize(&job.info).expect("deserializing JobInfo");

            let uid = store.set(job.pdf).expect("saving file in store");

            let hex_uid = hex::encode(&uid[..]);

            let task = DispatcherTask {
                user_id: user.id,
                uid,
                filename: info.filename,
                preprocess: 2,
                keep: Some(options.keep),
                a3: Some(options.a3),
                color: Some(options.color),
                duplex: Some(options.duplex),
                copies: Some(options.copies),
            };

            taskqueue.send(&task).expect("sending task to queue");

            info!("{} processing job with uid {} with image option", user.id, hex_uid);

            Ok(Status::new(202, "Started Processing"))
        }
        else {
            insert_into_jobs(
                user.id,
                job.info,
                job.options,
                job.pdf,
                job.preview_0,
                job.preview_1,
                job.preview_2,
                job.preview_3,
                &user.connection,
            )?;

            Ok(Status::new(200, "OK"))
        }
    }
    else {
        Ok(Status::new(404, "Job not found"))
    }
}

#[post("/sharecode", data = "<code>")]
pub fn post_sharecode(
    user: UserGuard,
    code: Json<String>,
    share: State<Share>,
) -> QueryResult<Status>
{
    if let Ok(id) = share.get(code.into_inner()) {
        if let Ok(job) = select_full_job_by_id(id, &user.connection) {
            insert_into_jobs(
                user.id,
                job.info,
                job.options,
                job.pdf,
                job.preview_0,
                job.preview_1,
                job.preview_2,
                job.preview_3,
                &user.connection,
            )?;

            Ok(Status::new(200, "OK"))
        }
        else {
            Ok(Status::new(404, "Job not found"))
        }
    }
    else {
        Ok(Status::new(404, "Sharecode not found"))
    }
}
