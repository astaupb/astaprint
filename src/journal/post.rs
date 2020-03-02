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
use rocket::{
    http::Status,
    response::status::Custom,
};
use rocket_contrib::json::Json;

use diesel::prelude::*;

use user::guard::UserGuard;

use mysql::{
    journal::{
        select::select_journal_token_by_content,
        JournalToken,
    },
    update_credit_with_unused_token,
};

#[post("/", data = "<token>")]
pub fn post_to_journal_with_token(user: UserGuard, token: Json<String>) -> QueryResult<Custom<()>>
{
    let token: Option<JournalToken> =
        select_journal_token_by_content(token.into_inner(), &user.connection)
            .expect("selecting journal token");

    match token {
        None => Ok(Custom(Status::new(401, "Unauthorized"), ())),
        Some(token) => {
            if token.used {
                return Ok(Custom(Status::new(472, "Token Already Consumed"), ()))
            }
            update_credit_with_unused_token(user.id, token.id, &user.connection)?;

            Ok(Custom(Status::new(204, "Success - No Content"), ()))
        },
    }
}
