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
use diesel::prelude::*;
use rocket_contrib::json::Json;

use user::guard::UserGuard;

use model::journal::JournalResponse;

use mysql::{
    select_full_journal_of_user,
};

#[get("/?<offset>&<limit>")]
pub fn get_journal_as_user(
    offset: Option<i64>,
    limit: Option<i64>,
    user: UserGuard,
) -> QueryResult<Json<Vec<JournalResponse>>>
{
    Ok(Json(
        select_full_journal_of_user(
            user.id,
            limit.unwrap_or(i64::from(u16::max_value()) * 2),
            offset.unwrap_or(0),
            &user.connection,
        )?
        .iter()
        .map(JournalResponse::from)
        .collect(),
    ))
}
