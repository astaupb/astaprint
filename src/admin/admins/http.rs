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

use chrono::NaiveDate;
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
        select_admin_by_login,
    },
    update::{
        update_admin,
        update_admin_hash_and_salt_by_id,
    },
};

use admin::{
    admins::{
        AdminCreate,
        AdminResponse,
        AdminUpdate,
        NewAdmin,
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
    if select_admin_by_login(&new.login, &admin.connection).is_ok() {
        return Ok(Custom(Status::new(472, "login already taken"), ()))
    }
    let new = new.into_inner();

    let (hash, salt) = PasswordHash::create(&new.password);

    let new_admin = AdminCreate {
        first_name: new.first_name,
        last_name: new.last_name,
        login: new.login,
        hash: hash,
        salt: salt,
        service: false,
        locked: false,
        created_by: Some(admin.id),
        expires: NaiveDate::from_yo(2019, 1),
    };

    new_admin.insert(&admin.connection)?;

    Ok(Custom(Status::new(204, "Success - Reset Content"), ()))
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
