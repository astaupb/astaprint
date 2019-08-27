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
use diesel::QueryResult;

use chrono::NaiveDateTime;

use rocket_contrib::json::Json;

use model::job::Job;
use mysql::jobs::select::*;
use user::guard::UserGuard;

#[get("/<id>")]
pub fn fetch_job(user: UserGuard, id: u32) -> QueryResult<Option<Json<Job>>>
{
    let job: Option<(u32, Vec<u8>, Vec<u8>, NaiveDateTime, NaiveDateTime)> =
        select_job_of_user(user.id, id, &user.connection)?;

    Ok(job.map(|x| Json(Job::from(x))))
}

#[get("/")]
pub fn jobs(user: UserGuard) -> QueryResult<Json<Vec<Job>>>
{
    let jobs: Vec<(u32, Vec<u8>, Vec<u8>, NaiveDateTime, NaiveDateTime)> =
        select_all_jobs_of_user(user.id, &user.connection)?;

    Ok(Json(jobs.iter().map(|x| Job::from(x.clone())).collect()))
}

#[get("/<id>/pdf")]
pub fn fetch_pdf(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_pdf(id, user.id, &user.connection).expect("selecting pdf"))
}

#[get("/<id>/preview/0")]
pub fn fetch_preview_0(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_preview_0(id, user.id, &user.connection).expect("selecting preview 0"))
}

#[get("/<id>/preview/1")]
pub fn fetch_preview_1(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_preview_1(id, user.id, &user.connection).expect("selection preview 1"))
}

#[get("/<id>/preview/2")]
pub fn fetch_preview_2(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_preview_2(id, user.id, &user.connection).expect("selection preview 2"))
}

#[get("/<id>/preview/3")]
pub fn fetch_preview_3(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    Ok(select_preview_3(id, user.id, &user.connection).expect("selection preview 2"))
}
