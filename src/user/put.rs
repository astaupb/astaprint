/// AStAPrint-Backend - User PUT Routes
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as
/// published by the Free Software Foundation, either version 3 of the
/// License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this program.  If not, see <https://www.gnu.org/licenses/>.
use rocket::http::Status;

use rocket_contrib::json::Json;

use diesel::QueryResult;

use sodium::PasswordHash;

use crate::user::guard::UserGuard;

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
    let (hash, salt): (Vec<u8>, Vec<u8>) =
        select_hash_and_salt(user.id, &user.connection)?;

    if PasswordHash::with_salt(&body.password.old, &salt[..]) == hash {
        let (hash, salt) = PasswordHash::create(&body.password.new);

        update_hash_and_salt(user.id, hash, salt, &user.connection)?;

        info!("{} changed password", user.id);

        Ok(Status::new(204, "No Content"))
    } else {
        info!("{} delivered wrong old password", user.id);

        Ok(Status::new(400, "Wrong Old Password"))
    }
}


#[put("/name", data = "<new_username>")]
pub fn change_username(
    user: UserGuard,
    new_username: Json<String>,
) -> QueryResult<Status>
{
    update_user_name(user.id, &new_username, &user.connection)?;

    info!("{} changed username", user.id);

    Ok(Status::new(205, "Reset Content"))
}
