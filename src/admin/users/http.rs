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
    users::{
        Card,
        UserResponse,
    },
};
use diesel::prelude::*;

use model::journal::JournalResponse;
use rocket::http::Status;
use rocket_contrib::json::Json;
use sodium::{
    random_bytes,
    PasswordHash,
};

use mysql::{
    select_full_journal_of_user,
    user::{
        delete::delete_all_tokens_of_user,
        select::{
            select_user_by_id,
            select_user_credit_by_id,
            select_user_email_by_id,
            select_user_with_limit_offset,
        },
        update::{
            update_hash_and_salt,
            update_locked,
            update_tou_accept,
            update_user_card_and_pin,
            update_user_email,
            update_user_name,
        },
    },
};

#[get("/?<limit>&<offset>")]
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

#[get("/<id>")]
pub fn get_user_as_admin(id: u32, admin: AdminGuard) -> QueryResult<Json<UserResponse>>
{
    Ok(Json(UserResponse::from(&select_user_by_id(id, &admin.connection)?)))
}

#[get("/<id>/credit")]
pub fn get_user_credit_as_admin(id: u32, admin: AdminGuard) -> QueryResult<Json<i32>>
{
    Ok(Json(select_user_credit_by_id(id, &admin.connection)?))
}

#[get("/<id>/journal?<offset>&<limit>")]
pub fn get_user_journal_as_admin(
    id: u32,
    offset: Option<i64>,
    limit: Option<i64>,
    admin: AdminGuard,
) -> QueryResult<Json<Vec<JournalResponse>>>
{
    Ok(Json(
        select_full_journal_of_user(
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

#[post("/<id>/password")]
pub fn reset_user_password_as_admin(admin: AdminGuard, id: u32) -> QueryResult<Status>
{
    if let Some(email) = select_user_email_by_id(id, &admin.connection)? {
        let password = base64::encode(&random_bytes(6));
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

#[put("/<id>/password", data = "<body>")]
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

#[put("/<id>/name", data = "<body>")]
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

#[put("/<id>/card", data = "<card>")]
pub fn change_user_card_as_admin(
    admin: AdminGuard,
    id: u32,
    card: Json<Card>,
) -> QueryResult<Status>
{
    let card = card.into_inner();

    update_user_card_and_pin(id, card.sn, card.pin, &admin.connection)?;

    Ok(Status::new(205, "Reset Content"))
}

#[put("/<id>/email", data = "<email>")]
pub fn change_user_email_as_admin(
    admin: AdminGuard,
    id: u32,
    email: Json<String>,
) -> QueryResult<Status>
{
    update_user_email(id, Some(email.into_inner()), &admin.connection)?;

    Ok(Status::new(205, "Reset Content"))
}

#[put("/<id>/locked", data = "<locked>")]
pub fn change_user_locked(admin: AdminGuard, id: u32, locked: Json<bool>) -> Status
{
    let locked = locked.into_inner();
    match update_locked(id, locked, &admin.connection) {
        Ok(1) => {
            info!("user {} locked: {}", id, locked);
            Status::new(205, "Success - Reset Content")
        },
        err => {
            error!("{:?}", err);
            Status::new(500, "Internal Server Error")
        },
    }
}

#[put("/tou_accept")]
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
