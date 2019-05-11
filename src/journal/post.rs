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

use admin::guard::AdminGuard;
use user::guard::UserGuard;

use sodium::random_bytes;

use mysql::{
    journal::{
        insert::insert_into_journal_token,
        select::select_journal_token_by_content,
        JournalToken,
    },
    update_credit_as_admin,
    update_credit_with_unused_token,
};

#[derive(Deserialize, Debug, Clone)]
pub struct JournalPost
{
    user_id: u32,
    value: i32,
    description: String,
    without_receipt: bool,
}

#[post("/", data = "<token>")]
pub fn post_to_journal_with_token(
    user: UserGuard,
    token: Json<String>,
) -> QueryResult<Custom<()>>
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

#[post("/journal", data = "<body>")]
pub fn post_to_journal_as_admin(
    body: Json<JournalPost>,
    admin: AdminGuard,
) -> QueryResult<Status>
{
    update_credit_as_admin(
        body.user_id,
        body.value,
        admin.id,
        &body.description,
        &admin.connection,
    )?;

    Ok(Status::new(204, "Success - No Content"))
}

#[post("/journal/tokens?<value>")]
pub fn post_journal_token_as_admin(
    value: u32,
    admin: AdminGuard,
) -> QueryResult<Json<String>>
{
    let content = base64::encode_config(&random_bytes(12)[..], base64::URL_SAFE);
    insert_into_journal_token(value, content.clone(), false, &admin.connection)?;

    Ok(Json(content))
}
