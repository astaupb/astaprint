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
use rocket::http::Status;
use rocket_contrib::json::Json;

use diesel::prelude::QueryResult;
use model::admin::AdminTokenResponse;
use mysql::admin::{
    delete::{
        delete_admin_token_by_id,
        delete_admin_tokens_by_admin_id,
    },
    select::{
        select_admin_tokens_by_admin_id,
        select_single_admin_token_optional,
    },
};

use crate::admin::{
    guard::AdminGuard,
    login::AdminLoginGuard,
};

#[get("/")]
pub fn get_admin_tokens(admin: AdminGuard) -> QueryResult<Json<Vec<AdminTokenResponse>>>
{
    Ok(Json(
        select_admin_tokens_by_admin_id(admin.id, &admin.connection)?
            .iter()
            .map(AdminTokenResponse::from)
            .collect(),
    ))
}

#[post("/")]
pub fn post_admin_token(guard: AdminLoginGuard) -> Json<String> { Json(guard.token) }

#[delete("/")]
pub fn delete_admin_tokens(admin: AdminGuard) -> QueryResult<Status>
{
    if delete_admin_tokens_by_admin_id(admin.id, &admin.connection)? > 0 {
        Ok(Status::new(205, "Success - Reset Content"))
    }
    else {
        Ok(Status::new(500, "Internal Server Error"))
    }
}

#[get("/<id>")]
pub fn get_single_admin_token(
    id: u32,
    admin: AdminGuard,
) -> QueryResult<Json<Option<AdminTokenResponse>>>
{
    Ok(Json(
        select_single_admin_token_optional(id, admin.id, &admin.connection)?
            .map(|row| AdminTokenResponse::from(&row)),
    ))
}

#[delete("/<id>")]
pub fn delete_single_admin_token(id: u32, admin: AdminGuard) -> QueryResult<Status>
{
    if delete_admin_token_by_id(id, admin.id, &admin.connection)? > 0 {
        Ok(Status::new(205, "Success - Reset Content"))
    }
    else {
        Ok(Status::new(500, "Internal Server Error"))
    }
}
