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

use admin::{
    admins::AdminUpdate,
    guard::AdminGuard,
};

use mysql::admin::{
    select::select_admin_by_id,
    update::{
        update_admin,
        update_admin_hash_and_salt_by_id,
    },
};

use sodium::PasswordHash;

use rocket::http::Status;
use rocket_contrib::json::Json;

use diesel::prelude::QueryResult;

#[put("/", data = "<update>")]
pub fn put_admin(admin: AdminGuard, update: Json<AdminUpdate>) -> QueryResult<Status>
{
    let old = select_admin_by_id(admin.id, &admin.connection)?;

    let updated = update.into_inner().update(old);

    Ok(if update_admin(updated, &admin.connection)? == 1 {
        Status::new(205, "Success - Reset Content")
    } else {
        Status::new(500, "Internal Server Error")
    })
}

#[put("/<id>/password", data = "<password>")]
pub fn put_admin_password(admin: AdminGuard, id: u32, password: Json<String>) -> QueryResult<Status>
{
    let (hash, salt) = PasswordHash::create(&password.into_inner());

    Ok(if update_admin_hash_and_salt_by_id(id, hash, salt, &admin.connection)? == 1 {
        Status::new(204, "Success - No Content")
    } else {
        Status::new(500, "Internal Server Error")
    })
}
