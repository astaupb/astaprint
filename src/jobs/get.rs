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
    result::Error,
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
        .first(&user.connection)
        .optional()?;

    Ok(job.map(|x| Json(JobResponse::from(x))))

}

#[get("/")]
fn jobs(user: UserGuard) -> Result<Json<Vec<JobResponse>>, diesel::result::Error>
{
    let jobs: Vec<(u32, u32, NaiveDateTime, Vec<u8>, Vec<u8>)> = jobs::table
        .select((
            jobs::id,
            jobs::user_id,
            jobs::created,
            jobs::info,
            jobs::options,
        ))
        .load(&user.connection)?;

    Ok(Json(jobs.iter().map(|x| JobResponse::from(x.clone())).collect()))
}

/*
#[get("/<id>/pdf")]
fn fetch_pdf<'a>(user: UserGuard, id: u32) -> diesel::result::QueryResult<Option<Stream<&'static [u8]>>>
{
    let pdf: Option<Vec<u8>> = jobs::table
        .select(jobs::data)
        .first(&user.connection)
        .optional()?;

}

#[get("/<id>/preview/<index>")]
fn fetch_preview(user: User, id: u32, index: u8) -> Option<NamedFile>
{
}

*/
