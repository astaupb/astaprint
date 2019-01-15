/// AStAPrint Jobs - options PUT
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as
/// published by the Free Software Foundation, either version 3 of the
/// License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::str::FromStr;

use diesel::result::QueryResult;

use rocket::http::Status;
use rocket_contrib::json::Json;

use model::job::options::JobOptions;

use jobs::{
    options::{
        pagerange::PageRange,
        JobOptionsUpdate,
        Update,
        Value::{
            self,
            B,
            I,
            S,
        },
    },
    *,
};

use mysql::jobs::update::*;

use user::guard::UserGuard;

#[put("/<id>/options/<option>", data = "<value>")]
pub fn update_single_option(
    user: UserGuard,
    id: u32,
    option: String,
    value: Json<Value>,
) -> QueryResult<Result<Option<Status>, Status>>
{
    let result: Option<Vec<u8>> =
        select_job_options(id, user.id, &user.connection)?;

    let mut options: JobOptions = match result {
        None => return Ok(Ok(None)),
        Some(options) => {
            bincode::deserialize(&options[..]).expect("deserializing JobOptions")
        },
    };
    match (option.as_ref(), value.into_inner()) {
        ("duplex", I(value)) => {
            options.duplex = value as u8;
        },
        ("copies", I(value)) => {
            options.copies = value;
        },
        ("collate", B(value)) => {
            options.collate = value;
        },
        ("keep", B(value)) => {
            options.keep = value;
        },
        ("a3", B(value)) => {
            options.a3 = value;
        },
        ("range", S(value)) => {
            if PageRange::from_str(&value).is_ok() {
                options.range = value;
            }
        },
        ("nup", I(value)) => {
            options.nup = value as u8;
        },
        ("nuppageorder", I(value)) => {
            options.nuppageorder = value as u8;
        },
        (_option, _) => {
            return Ok(Err(Status::new(400, "Bad Request")));
        },
    };
    let value = bincode::serialize(&options).expect("serializing JobOptions");

    update_job_options(id, user.id, value, &user.connection)?;

    Ok(Ok(Some(Status::new(205, "Reset Content"))))
}

#[put("/<id>/options", data = "<options_update>")]
pub fn update_options(
    user: UserGuard,
    id: u32,
    options_update: Json<JobOptionsUpdate>,
) -> QueryResult<Result<Option<Status>, Status>>
{
    let result: Option<Vec<u8>> =
        select_job_options(id, user.id, &user.connection)?;

    let mut options: JobOptions = match result {
        None => return Ok(Ok(None)),
        Some(options) => {
            bincode::deserialize(&options[..]).expect("deserializing JobOptions")
        },
    };

    options.merge(options_update.into_inner());

    let serialized = bincode::serialize(&options).expect("serializing JobOptions");

    update_job_options(id, user.id, serialized, &user.connection)?;

    Ok(Ok(Some(Status::new(205, "Reset Content"))))
}
