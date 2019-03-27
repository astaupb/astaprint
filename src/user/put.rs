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

use rocket::response::status::Custom;
use rocket_contrib::json::Json;

use diesel::QueryResult;

use sodium::PasswordHash;

use crate::user::guard::UserGuard;

use crate::admin::guard::AdminGuard;

use mysql::user::{
    select::*,
    update::*,
};

#[derive(Deserialize, Debug)]
struct PasswordChange
{
    old: String,
    new: String,
}

#[derive(Deserialize, Debug)]
pub struct PasswordChangeBody
{
    password: PasswordChange,
}

#[put("/password", data = "<body>")]
pub fn change_password(
    user: UserGuard,
    body: Json<PasswordChangeBody>,
) -> QueryResult<Status>
{
    let (hash, salt): (Vec<u8>, Vec<u8>) = select_hash_and_salt(user.id, &user.connection)?;

    if PasswordHash::with_salt(&body.password.old, &salt[..]) == hash {
        let (hash, salt) = PasswordHash::create(&body.password.new);

        update_hash_and_salt(user.id, hash, salt, &user.connection)?;

        info!("{} changed password", user.id);

        Ok(Status::new(204, "No Content"))
    }
    else {
        info!("{} delivered wrong old password", user.id);

        Ok(Status::new(400, "Wrong Old Password"))
    }
}

#[put("/name", data = "<new_username>")]
pub fn change_username(
    user: UserGuard,
    new_username: Json<String>,
) -> QueryResult<Custom<()>>
{
    let name = new_username.into_inner();

    if name.chars().any(|c| !c.is_alphanumeric()) || name.bytes().count() > 32 {
        return Ok(Custom(Status::new(471, "Invalid Username"), ()))
    }

    let user_id = select_user_id_by_name(&name, &user.connection)?;

    if user_id.is_some() {
        return Ok(Custom(Status::new(472, "Username Already Taken"), ()))
    }

    update_user_name(user.id, &name, &user.connection)?;

    info!("user#{} changed username", user.id);

    Ok(Custom(Status::new(205, "Reset Content"), ()))
}
#[derive(Deserialize, Debug, Clone)]
pub struct Card
{
    sn: Option<u64>,
    pin: Option<u32>,
}

#[put("/<user_id>/card", data = "<card>")]
pub fn change_card(admin: AdminGuard, user_id: u32, card: Json<Card>) -> QueryResult<Status>
{
    let card = card.into_inner();

    update_user_card_and_pin(user_id, card.sn, card.pin, &admin.connection)?;

    Ok(Status::new(205, "Reset Content"))
}
