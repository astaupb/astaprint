/// AStAPrint Jobs - info GET
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
    prelude::*,
    result::QueryResult,
};

use rocket_contrib::Json;

use jobs::{
    info::JobInfo,
    *,
};

use user::guard::UserGuard;

#[get("/<id>/info")]
fn fetch_info(user: UserGuard, id: u32) -> QueryResult<Option<Json<JobInfo>>>
{
    let result: Option<Vec<u8>> = jobs::table
        .select(jobs::info)
        .filter(jobs::user_id.eq(user.id))
        .filter(jobs::id.eq(id))
        .first(&user.connection)
        .optional()?;

    Ok(result.map(|serialized| {
        let info: JobInfo = bincode::deserialize(&serialized[..]).expect("deserializing JobInfo");
        Json(info)
    }))
}