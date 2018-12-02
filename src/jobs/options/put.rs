/// AStAPrint Jobs - options PUT
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
use std::str::FromStr;

use diesel::{
    prelude::*,
    result::QueryResult,
    update,
};

use rocket_contrib::Json;

use rocket::response::status::{
    BadRequest,
    Reset,
};

use jobs::{
    options::{
        pagerange::PageRange,
        JobOptions,
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

use user::guard::UserGuard;

#[put("/<id>/options/<option>", data = "<value>")]
fn update_single_option(
    user: UserGuard,
    id: u32,
    option: String,
    value: Json<Value>,
) -> QueryResult<Result<Option<Reset>, BadRequest<String>>>
{
    let result: Option<Vec<u8>> = jobs::table
        .select(jobs::options)
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)
        .optional()?;

    let mut options: JobOptions = match result {
        None => return Ok(Ok(None)),
        Some(options) => bincode::deserialize(&options[..]).expect("deserializing JobOptions"),
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
        (option, _) => {
            return Ok(Err(BadRequest(Some(format!("{} is unknown or of the wrong type", option)))));
        },
    };
    let value = bincode::serialize(&options).expect("serializing JobOptions");

    update(jobs::table.filter(jobs::user_id.eq(user.id)).filter(jobs::id.eq(id)))
        .set(jobs::options.eq(value))
        .execute(&user.connection)?;

    Ok(Ok(Some(Reset)))
}

#[put("/<id>/options", data = "<options_update>")]
fn update_options(
    user: UserGuard,
    id: u32,
    options_update: Json<JobOptionsUpdate>,
) -> QueryResult<Result<Option<Reset>, BadRequest<String>>>
{
    let result: Option<Vec<u8>> = jobs::table
        .select(jobs::options)
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)
        .optional()?;

    let mut options: JobOptions = match result {
        None => return Ok(Ok(None)),
        Some(options) => bincode::deserialize(&options[..]).expect("deserializing JobOptions"),
    };

    options.merge(options_update.into_inner());

    let serialized = bincode::serialize(&options).expect("serializing JobOptions");

    update(jobs::table.filter(jobs::id.eq(id)).filter(jobs::user_id.eq(user.id)))
        .set(jobs::options.eq(serialized))
        .execute(&user.connection)?;

    Ok(Ok(Some(Reset)))
}
