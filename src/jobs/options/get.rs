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

use rocket_contrib::json::Json;

use jobs::*;

use user::guard::UserGuard;

#[get("/<id>/options")]
pub fn fetch_options(user: UserGuard, id: u32) -> QueryResult<Option<Json<JobOptions>>>
{
    let result: Option<Vec<u8>> = select_job_options(id, user.id, &user.connection)?;

    Ok(result.map(|serialized| {
        Json(JobOptions::from(&serialized[..]))
    }))
}
