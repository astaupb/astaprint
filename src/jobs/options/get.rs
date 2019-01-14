/// AStAPrint Jobs - options GET
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
use diesel::result::QueryResult;

use rocket_contrib::json::Json;

use jobs::{
    options::Value,
    *,
};

use user::guard::UserGuard;

#[get("/<id>/options/<option>")]
pub fn fetch_single_option(
    user: UserGuard,
    id: u32,
    option: String,
) -> QueryResult<Option<Json<Value>>>
{
    let result: Option<Vec<u8>> =
        select_job_options(id, user.id, &user.connection)?;

    Ok(result.and_then(|serialized| {
        let options: JobOptions = bincode::deserialize(&serialized[..])
            .expect("deserializing JobOptions");

        match option.as_ref() {
            "duplex" => Some(Json(Value::I(u16::from(options.duplex)))),
            "copies" => Some(Json(Value::I(options.copies))),
            "collate" => Some(Json(Value::B(options.collate))),
            "keep" => Some(Json(Value::B(options.keep))),
            "a3" => Some(Json(Value::B(options.a3))),
            "range" => Some(Json(Value::S(options.range))),
            "nup" => Some(Json(Value::I(u16::from(options.nup)))),
            "nuppageorder" => {
                Some(Json(Value::I(u16::from(options.nuppageorder))))
            },
            &_ => None,
        }
    }))
}

#[get("/<id>/options")]
pub fn fetch_options(
    user: UserGuard,
    id: u32,
) -> QueryResult<Option<Json<JobOptions>>>
{
    let result: Option<Vec<u8>> =
        select_job_options(id, user.id, &user.connection)?;

    Ok(result.map(|serialized| {
        let options: JobOptions = bincode::deserialize(&serialized[..])
            .expect("deserializing JobOptions");
        Json(options)
    }))
}
