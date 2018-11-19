/// AStAPrint - Jobs GET Routes
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

use diesel::{
    result::{QueryResult, Error},
    prelude::*,
};

use rocket::{
    response::{
        status::{
            Accepted,
            BadRequest,
            Reset,
        },
        Stream,
    },
    State,
};
use rocket_contrib::Json;

use jobs::*;
use jobs::response::JobResponse;
use user::guard::UserGuard;

#[get("/<id>")]
fn fetch_job(user: UserGuard, id: u32) -> Result<Option<Json<JobResponse>>, diesel::result::Error>
{
    let job: Option<(u32, u32, NaiveDateTime, Vec<u8>, Vec<u8>)> = jobs::table
        .select((
            jobs::id,
            jobs::user_id,
            jobs::created,
            jobs::info,
            jobs::options,
        ))
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)
        .optional()?;

    Ok(job.map(|x| Json(JobResponse::from(x))))

}

#[get("/")]
fn jobs(user: UserGuard) -> QueryResult<Json<Vec<JobResponse>>>
{
    let jobs: Vec<(u32, u32, NaiveDateTime, Vec<u8>, Vec<u8>)> = jobs::table
        .select((
            jobs::id,
            jobs::user_id,
            jobs::created,
            jobs::info,
            jobs::options,
        ))
        .filter(jobs::user_id.eq(user.id))
        .load(&user.connection)?;

    Ok(Json(jobs.iter().map(|x| JobResponse::from(x.clone())).collect()))
}

#[get("/<id>/pdf")]
fn fetch_pdf(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    let pdf: Option<Vec<u8>> = jobs::table
        .select(jobs::data)
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)
        .optional()?;

    Ok(pdf)

}

#[get("/<id>/preview/0")]
fn fetch_preview_0(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    let preview = jobs::table
        .select(jobs::preview_0)
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)
        .optional()?;

    Ok(preview)
}

#[get("/<id>/preview/1")]
fn fetch_preview_1(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    let preview = jobs::table
        .select(jobs::preview_1)
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)?;

    Ok(preview)
}

#[get("/<id>/preview/2")]
fn fetch_preview_2(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    let preview = jobs::table
        .select(jobs::preview_2)
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)?;

    Ok(preview)
}

#[get("/<id>/preview/3")]
fn fetch_preview_3(user: UserGuard, id: u32) -> QueryResult<Option<Vec<u8>>>
{
    let preview = jobs::table
        .select(jobs::preview_3)
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)?;

    Ok(preview)
}
