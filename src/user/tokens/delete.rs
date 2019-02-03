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
use rocket::http::Status;
use user::guard::UserGuard;

use mysql::user::delete::*;

#[delete("/")]
pub fn delete_all_tokens(user: UserGuard) -> QueryResult<Status>
{
    let deleted = delete_all_tokens_of_user(user.id, &user.connection)?;
    info!("{} deleted {} tokens", user.id, deleted);

    Ok(Status::new(205, "Reset Content"))
}

#[delete("/<token_id>")]
pub fn delete_single_token(
    user: UserGuard,
    token_id: u32,
) -> QueryResult<Option<Status>>
{
    let affected_rows = delete_user_token_by_id(user.id, token_id, &user.connection)?;
    if affected_rows > 0 {
        info!("{} deleted token {}", user.id, &token_id);
        Ok(Some(Status::new(205, "Reset Content")))
    }
    else {
        Ok(None)
    }
}
