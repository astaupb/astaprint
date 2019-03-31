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
use legacy::tds::{
    get_credit,
    get_journal_of_user,
};
use model::{
    job::options::JobOptions,
    journal::Transaction,
};
use mysql::{
    journal::{
        select::select_journal_tokens,
        JournalToken,
    },
    user::{
        select::{
            select_user_by_id,
            select_user_with_limit_offset,
        },
        User,
    },
};

use bigdecimal::{
    BigDecimal,
    ToPrimitive,
};

use rocket_contrib::json::Json;

#[derive(Serialize, Debug, Clone)]
pub struct UserResponse
{
    pub id: u32,
    pub name: String,
    pub options: Option<JobOptions>,
    pub card: Option<u64>,
    pub pin: Option<u32>,
    pub locked: bool,
    pub created: String,
    pub updated: String,
}

impl<'a> From<&'a User> for UserResponse
{
    fn from(user: &User) -> UserResponse
    {
        UserResponse {
            id: user.id,
            name: user.name.clone(),
            options: user
                .options
                .clone()
                .map(|x| bincode::deserialize(&x[..]).expect("deserializing JobOption")),
            card: user.card,
            pin: user.pin,
            locked: user.locked,
            created: format!("{}", user.created),
            updated: format!("{}", user.updated),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct JournalTokenResponse
{
    id: u32,
    value: u32,
    content: String,
    used: bool,
    used_by: Option<u32>,
    created: String,
    updated: String,
}

impl<'a> From<&'a JournalToken> for JournalTokenResponse
{
    fn from(token: &JournalToken) -> JournalTokenResponse
    {
        JournalTokenResponse {
            id: token.id,
            value: (token.value.clone() * BigDecimal::from(100)).to_u32().unwrap(),
            content: token.content.clone(),
            used: token.used,
            used_by: token.used_by,
            created: format!("{}", token.created),
            updated: format!("{}", token.updated),
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
pub fn get_user_as_admin(
    id: u32,
    admin: AdminGuard,
) -> QueryResult<Json<UserResponse>>
{
    Ok(Json(UserResponse::from(&select_user_by_id(id, &admin.connection)?)))
}

#[get("/users/<id>/journal?<desc>&<offset>&<limit>")]
pub fn get_user_journal_as_admin(
    id: u32,
    desc: Option<bool>,
    offset: Option<i32>,
    limit: Option<u32>,
    _admin: AdminGuard,
) -> Json<Vec<Transaction>>
{
    Json(get_journal_of_user(
        id,
        desc.unwrap_or(true),
        offset.unwrap_or(0),
        limit.unwrap_or(u32::max_value()),
    ))
}

#[get("/users/<id>/credit")]
pub fn get_user_credit_as_admin(id: u32, _admin: AdminGuard) -> Json<i32> { Json(get_credit(id)) }

#[get("/journal/tokens")]
pub fn get_journal_tokens_as_admin(
    admin: AdminGuard
) -> QueryResult<Json<Vec<JournalTokenResponse>>>
{
    Ok(Json(
        select_journal_tokens(&admin.connection)?.iter().map(JournalTokenResponse::from).collect(),
    ))
}
