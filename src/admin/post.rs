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
    email::send_password_reset_email,
    guard::AdminGuard,
};

use base64::encode;
use diesel::prelude::*;
use mysql::{
    user::{
        delete::delete_all_tokens_of_user,
        select::select_user_email_by_id,
        update::{
            update_hash_and_salt,
            update_tou_accept,
            update_user_name,
        },
    },
};
use rocket::{
    http::Status,
};

use rocket_contrib::json::Json;
use sodium::{
    random_bytes,
    PasswordHash,
};

#[put("/users/<id>/password", data = "<body>")]
pub fn change_user_password_as_admin(
    admin: AdminGuard,
    id: u32,
    body: Json<String>,
) -> QueryResult<Status>
{
    let password = body.into_inner();

    let (hash, salt) = PasswordHash::create(&password);

    update_hash_and_salt(id, hash, salt, &admin.connection)?;

    delete_all_tokens_of_user(id, &admin.connection)?;

    info!("{} changed password of user {}", admin.id, id);

    Ok(Status::new(204, "No Content"))
}

#[post("/users/<id>/password")]
pub fn reset_user_password_as_admin(admin: AdminGuard, id: u32) -> QueryResult<Status>
{
    if let Some(email) = select_user_email_by_id(id, &admin.connection)? {
        let password = encode(&random_bytes(6));
        if send_password_reset_email(&email, &password).is_ok() {
            let (hash, salt) = PasswordHash::create(&password);
            update_hash_and_salt(id, hash, salt, &admin.connection)?;

            delete_all_tokens_of_user(id, &admin.connection)?;

            Ok(Status::new(204, "No Content"))
        }
        else {
            Ok(Status::new(500, "Unable to deliver email"))
        }
    }
    else {
        Ok(Status::new(400, "User has no email"))
    }
}

#[put("/users/<id>/name", data = "<body>")]
pub fn change_user_name_as_admin(
    admin: AdminGuard,
    id: u32,
    body: Json<String>,
) -> QueryResult<Status>
{
    let name = body.into_inner();

    update_user_name(id, &name, &admin.connection)?;

    info!("{} changed name of user {} ", admin.id, id);

    Ok(Status::new(205, "Reset Content"))
}

#[put("/users/tou_accept")]
pub fn clear_tou_accept(admin: AdminGuard) -> Status
{
    match update_tou_accept(false, &admin.connection) {
        Ok(1) => {
            info!("tou_accept set to 0");
            Status::new(204, "Success - No Content")
        },
        err => {
            error!("{:?}", err);
            Status::new(500, "Internal Server Error")
        },
    }
}
