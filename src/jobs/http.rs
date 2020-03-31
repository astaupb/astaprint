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
use diesel::result::QueryResult;

use rocket::{
    http::Status,
    State,
};
use rocket_contrib::json::Json;

use model::{
    job::{
        info::JobInfo,
        options::{
            pagerange::PageRange,
            update::{
                JobOptionsUpdate,
                Update,
            },
            JobOptions,
        },
        Job,
    },
    task::dispatcher::DispatcherTask,
};

use crate::{
    user::guard::UserGuard,
    jobs::queue::start_dispatch,
};

use redis::{
    queue::TaskQueueClient,
    share::Share,
    store::Store,
};

use mysql::jobs::{
    delete::*,
    insert::*,
    select::*,
    update::*,
    JobSelect,
};

#[get("/")]
pub fn jobs(user: UserGuard) -> QueryResult<Json<Vec<Job>>>
{
    let jobs: Vec<JobSelect> = select_all_jobs_of_user(user.id, &user.connection)?;

    Ok(Json(jobs.iter().map(|x| Job::from(x.clone())).collect()))
}

#[delete("/")]
pub fn delete_all_jobs(user: UserGuard) -> QueryResult<Status>
{
    let ids = select_job_ids_of_user(user.id, &user.connection)?;
    for id in ids {
        let _deleted = delete_job_of_user_by_id(user.id, id, &user.connection)?;
    }

    Ok(Status::new(205, "Reset Content"))
}

#[get("/<id>")]
pub fn fetch_job(user: UserGuard, id: u32) -> QueryResult<Option<Json<Job>>>
{
    let job: Option<JobSelect> = select_job_of_user(user.id, id, &user.connection)?;

    Ok(job.map(|x| Json(Job::from(x))))
}

#[delete("/<id>")]
pub fn delete_job(user: UserGuard, id: u32) -> QueryResult<Option<Status>>
{
    let deleted = delete_job_of_user_by_id(user.id, id, &user.connection)?;

    Ok(if deleted == 1 {
        Some(Status::new(205, "Reset Content"))
    }
    else {
        None
    })
}

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

#[get("/<id>/pdf")]
pub fn fetch_pdf(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_pdf(id, user.id, &user.connection)?)
}

#[get("/<id>/preview/0")]
pub fn fetch_preview_0(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_preview_0(id, user.id, &user.connection)?)
}

#[get("/<id>/preview/1")]
pub fn fetch_preview_1(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_preview_1(id, user.id, &user.connection)?)
}

#[get("/<id>/preview/2")]
pub fn fetch_preview_2(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_preview_2(id, user.id, &user.connection)?)
}

#[get("/<id>/preview/3")]
pub fn fetch_preview_3(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_preview_3(id, user.id, &user.connection)?)
}

#[get("/<id>/info")]
pub fn fetch_info(user: UserGuard, id: u32) -> QueryResult<Option<Json<JobInfo>>>
{
    let result: Option<Vec<u8>> = select_job_info(id, user.id, &user.connection)?;

    Ok(result.map(|serialized| {
        let info = JobInfo::from(&serialized[..]);
        Json(info)
    }))
}

#[get("/<id>/options")]
pub fn fetch_options(user: UserGuard, id: u32) -> QueryResult<Option<Json<JobOptions>>>
{
    let result: Option<Vec<u8>> = select_job_options(id, user.id, &user.connection)?;

    Ok(result.map(|serialized| Json(JobOptions::from(&serialized[..]))))
}

#[put("/<id>/options", data = "<update>")]
pub fn update_options(
    user: UserGuard,
    id: u32,
    update: Json<JobOptionsUpdate>,
) -> QueryResult<Status>
{
    if let Some((id, info, options, _created, _updated)) =
        select_job_of_user(user.id, id, &user.connection)?
    {
        let info: JobInfo = JobInfo::from(&info[..]);
        let mut options = JobOptions::from(&options[..]);

        options.merge(update.into_inner());

        if let Some(range) = PageRange::new(&options.range, info.pagecount as usize) {
            options.range = format!("{}", range);
            let _char = options.range.pop();
        }
        else {
            options.range = String::from("");
        }

        update_job_options(id, user.id, options.serialize(), &user.connection)?;

        Ok(Status::new(205, "Reset Content"))
    }
    else {
        Ok(Status::new(404, "Not Found"))
    }
}

#[get("/<id>/sharecode")]
pub fn get_sharecode(
    user: UserGuard,
    id: u32,
    share: State<Share>,
) -> QueryResult<Result<Json<String>, Status>>
{
    if let Some(id) = select_job_id_of_user(user.id, id, &user.connection)? {
        if let Ok(key) = share.set(id) {
            Ok(Ok(Json(key)))
        }
        else {
            Ok(Err(Status::new(500, "Internal Server Error")))
        }
    }
    else {
        Ok(Err(Status::new(403, "Forbidden")))
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
