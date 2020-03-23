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

use jobs::queue::start_dispatch;

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
            let options = JobOptions::from(&job.options[..]);

            let info = JobInfo::from(&job.info[..]);

            let _hex_uid = start_dispatch(
                user.id,
                job.pdf,
                Some(info.filename),
                Some(2),
                Some(options.keep),
                Some(options.a3),
                Some(options.color),
                Some(options.duplex),
                Some(options.copies),
                store.inner(),
                taskqueue.inner(),
            );

            Ok(Status::new(202, "Started Processing"))
        }
        else {
            insert_into_jobs(
                JobInsert {
                    user_id: user.id,
                    info: job.info,
                    options: job.options,
                    pdf: job.pdf,
                    preview_0: job.preview_0,
                    preview_1: job.preview_1,
                    preview_2: job.preview_2,
                    preview_3: job.preview_3,
                },
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
                JobInsert {
                    user_id: user.id,
                    info: job.info,
                    options: job.options,
                    pdf: job.pdf,
                    preview_0: job.preview_0,
                    preview_1: job.preview_1,
                    preview_2: job.preview_2,
                    preview_3: job.preview_3,
                },
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
