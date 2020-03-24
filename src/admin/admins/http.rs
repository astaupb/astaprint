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
use rocket::{
    http::Status,
    response::status::Custom,
};
use rocket_contrib::json::Json;
use sodium::PasswordHash;

use mysql::admin::{
    delete::delete_admin_by_id,
    select::{
        select_admin,
        select_admin_by_id,
    },
    update::{
        update_admin,
        update_admin_hash_and_salt_by_id,
    },
};

use model::admin::AdminResponse;

use admin::{
    admins::{
        add::{
            add_admin,
            AdminAddError::*,
            NewAdmin,
        },
        update::AdminUpdate,
    },
    guard::AdminGuard,
};

#[get("/")]
pub fn get_admins(admin: AdminGuard) -> QueryResult<Json<Vec<AdminResponse>>>
{
    Ok(Json(select_admin(&admin.connection)?.iter().map(AdminResponse::from).collect()))
}

#[post("/", data = "<new>")]
pub fn post_new_admin(admin: AdminGuard, new: Json<NewAdmin>) -> QueryResult<Custom<()>>
{
    match add_admin(new.into_inner(), Some(admin.id), &admin.connection) {
        Ok(_) => Ok(Custom(Status::new(205, "Success - Reset Content"), ())),
        Err(LoginInvalid) => Ok(Custom(Status::new(471, "Invalid login"), ())),
        Err(LoginExists) => Ok(Custom(Status::new(472, "login already taken"), ())),
        Err(QueryError(e)) => Err(e),
    }
}

#[get("/<id>")]
pub fn get_single_admin(admin: AdminGuard, id: u32) -> QueryResult<Json<AdminResponse>>
{
    Ok(Json(AdminResponse::from(&select_admin_by_id(id, &admin.connection)?)))
}

#[delete("/<id>")]
pub fn delete_admin(admin: AdminGuard, id: u32) -> QueryResult<Status>
{
    Ok(if delete_admin_by_id(id, &admin.connection)? == 1 {
        Status::new(205, "Success - Reset Content")
    }
    else {
        Status::new(500, "Internal Server Error")
    })
}

#[put("/<id>", data = "<update>")]
pub fn put_admin(admin: AdminGuard, id: u32, update: Json<AdminUpdate>) -> QueryResult<Status>
{
    let old = select_admin_by_id(id, &admin.connection)?;

    let updated = update.into_inner().update(old);

    Ok(if update_admin(updated, &admin.connection)? == 1 {
        Status::new(205, "Success - Reset Content")
    }
    else {
        Status::new(500, "Internal Server Error")
    })
}

#[put("/<id>/password", data = "<password>")]
pub fn put_admin_password(admin: AdminGuard, id: u32, password: Json<String>)
-> QueryResult<Status>
{
    let (hash, salt) = PasswordHash::create(&password.into_inner());

    Ok(if update_admin_hash_and_salt_by_id(id, hash, salt, &admin.connection)? == 1 {
        Status::new(204, "Success - No Content")
    }
    else {
        Status::new(500, "Internal Server Error")
    })
}
