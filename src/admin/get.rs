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
use admin::guard::AdminGuard;
use diesel::prelude::*;

use model::{
    job::options::JobOptions,
    journal::{
        JournalResponse,
        JournalTokenResponse,
    },
};

use mysql::{
    journal::select::{
        select_journal_of_user_with_limit_and_offset,
        select_journal_tokens,
    },
    user::{
        select::{
            select_user_by_id,
            select_user_credit_by_id,
            select_user_with_limit_offset,
        },
        User,
    },
};

use rocket_contrib::json::Json;

#[derive(Serialize, Debug, Clone)]
pub struct UserResponse
{
    pub id: u32,
    pub name: String,
    pub credit: i32,
    pub options: Option<JobOptions>,
    pub card: Option<u64>,
    pub pin: Option<u32>,
    pub locked: bool,
    pub created: i64,
    pub updated: i64,
}

impl<'a> From<&'a User> for UserResponse
{
    fn from(user: &User) -> UserResponse
    {
        UserResponse {
            id: user.id,
            name: user.name.clone(),
            credit: user.credit,
            options: user
                .options
                .clone()
                .map(|x| bincode::deserialize(&x[..]).expect("deserializing JobOption")),
            card: user.card,
            pin: user.pin,
            locked: user.locked,
            created: user.created.timestamp(),
            updated: user.updated.timestamp(),
        }
    }
}

#[get("/users?<limit>&<offset>")]
pub fn get_all_users(
    limit: Option<i64>,
    offset: Option<i64>,
    admin: AdminGuard,
) -> QueryResult<Json<Vec<UserResponse>>>
{
    Ok(Json(
        select_user_with_limit_offset(limit.unwrap_or(50), offset.unwrap_or(0), &admin.connection)?
            .iter()
            .map(UserResponse::from)
            .collect(),
    ))
}

#[get("/users/<id>")]
pub fn get_user_as_admin(id: u32, admin: AdminGuard) -> QueryResult<Json<UserResponse>>
{
    Ok(Json(UserResponse::from(&select_user_by_id(id, &admin.connection)?)))
}

#[get("/users/<id>/journal?<offset>&<limit>")]
pub fn get_user_journal_as_admin(
    id: u32,
    offset: Option<i64>,
    limit: Option<i64>,
    admin: AdminGuard,
) -> QueryResult<Json<Vec<JournalResponse>>>
{
    Ok(Json(
        select_journal_of_user_with_limit_and_offset(
            id,
            limit.unwrap_or_else(|| i32::max_value().into()),
            offset.unwrap_or(0),
            &admin.connection,
        )?
        .iter()
        .map(JournalResponse::from)
        .collect(),
    ))
}

#[get("/users/<id>/credit")]
pub fn get_user_credit_as_admin(id: u32, admin: AdminGuard) -> QueryResult<Json<i32>>
{
    Ok(Json(select_user_credit_by_id(id, &admin.connection)?))
}

#[get("/journal/tokens")]
pub fn get_journal_tokens_as_admin(
    admin: AdminGuard,
) -> QueryResult<Json<Vec<JournalTokenResponse>>>
{
    Ok(Json(
        select_journal_tokens(&admin.connection)?.iter().map(JournalTokenResponse::from).collect(),
    ))
}
