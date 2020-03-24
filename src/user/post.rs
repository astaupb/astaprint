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
    NewUser,
    UserAddError::*,
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

#[post("/", data = "<user>")]
pub fn register_as_new_user(
    user: Json<NewUser>,
    pool: State<Pool<ConnectionManager<MysqlConnection>>>,
) -> QueryResult<Custom<()>>
{
    match add_user(user.into_inner(), &pool.get().expect("getting mysql connection")) {
        Ok(_id) => Ok(Custom(Status::new(204, "Success - No Content"), ())),
        Err(UsernameExists) => Ok(Custom(Status::new(470, "username already taken"), ())),
        Err(UsernameInvalid) => Ok(Custom(Status::new(471, "Invalid Username"), ())),
        Err(EmailExists) => Ok(Custom(Status::new(472, "email already taken"), ())),
        Err(QueryError(e)) => Err(e),
    }
}
