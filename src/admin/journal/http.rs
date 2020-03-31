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

use diesel::prelude::QueryResult;
use model::journal::{
    JournalResponse,
    JournalTokenResponse,
};
use rocket::http::Status;
use rocket_contrib::json::Json;
use sodium::random_bytes;

use mysql::{
    journal::{
        insert::insert_into_journal_token,
        select::select_journal_tokens,
    },
    select_full_journal,
    update_credit_as_admin,
};

use crate::admin::{
    guard::AdminGuard,
    journal::JournalPost,
};

#[get("/?<offset>&<limit>")]
pub fn get_journal_as_admin(
    offset: Option<i64>,
    limit: Option<i64>,
    admin: AdminGuard,
) -> QueryResult<Json<Vec<JournalResponse>>>
{
    Ok(Json(
        select_full_journal(
            limit.unwrap_or(i64::from(u16::max_value()) * 2),
            offset.unwrap_or(0),
            &admin.connection,
        )?
        .iter()
        .map(JournalResponse::from)
        .collect(),
    ))
}

#[post("/", data = "<body>")]
pub fn post_to_journal_as_admin(body: Json<JournalPost>, admin: AdminGuard) -> QueryResult<Status>
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

#[get("/tokens")]
pub fn get_journal_tokens_as_admin(
    admin: AdminGuard,
) -> QueryResult<Json<Vec<JournalTokenResponse>>>
{
    Ok(Json(
        select_journal_tokens(&admin.connection)?.iter().map(JournalTokenResponse::from).collect(),
    ))
}

#[post("/tokens?<value>")]
pub fn post_journal_token_as_admin(value: u32, admin: AdminGuard) -> QueryResult<Json<String>>
{
    let content = base64::encode_config(&random_bytes(12)[..], base64::URL_SAFE);
    insert_into_journal_token(value, content.clone(), false, &admin.connection)?;

    Ok(Json(content))
}
