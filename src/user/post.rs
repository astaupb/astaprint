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
use rocket_contrib::json::Json;

use rocket::{
    http::Status,
    response::status::Custom,
    State,
};
use user::{
    guard::UserGuard,
    login::LoginGuard,
};

use user::add::{
    add_user,
    UserAddError,
};

use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use mysql::user::delete::*;

#[post("/tokens")]
pub fn login(login: LoginGuard) -> Json<String> { Json(login.token) }

#[post("/logout")]
pub fn logout(user: UserGuard) -> QueryResult<Status>
{
    delete_user_token_by_id(user.id, user.token_id, &user.connection)?;

    info!("{} logged out", user.id);

    Ok(Status::new(205, "Reset View"))
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewUser
{
    name: String,
    password: String,
    email: Option<String>,
}

#[post("/", data = "<user>")]
pub fn register_as_new_user(
    user: Json<NewUser>,
    pool: State<Pool<ConnectionManager<MysqlConnection>>>,
) -> QueryResult<Custom<()>>
{
    let connection = pool.get().expect("getting mysql connection from pool");

    if user.name.chars().any(|c| !c.is_alphanumeric()) || user.name.bytes().count() > 32 {
        return Ok(Custom(Status::new(471, "Invalid Username"), ()))
    }
    match add_user(&user.name, &user.password, user.email.clone(), false, &connection) {
        Ok(_id) => Ok(Custom(Status::new(204, "Success - No Content"), ())),
        Err(UserAddError::UsernameExists) => {
            Ok(Custom(Status::new(470, "username already taken"), ()))
        },
        Err(UserAddError::InsertError(e)) => Err(e),
    }
}
