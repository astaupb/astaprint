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
    guard::AdminGuard,
    Admin,
};
use chrono::NaiveDate;
use diesel::prelude::*;
use mysql::admin::select::select_admin_by_login;
use rocket::http::Status;
use rocket_contrib::json::Json;
use sodium::PasswordHash;
use user::add::{
    add_user,
    UserAddError,
};

#[derive(Deserialize, Debug, Clone)]
pub struct NewAdmin
{
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub password: String,
    pub owner: bool,
}

#[post("/", data = "<new>")]
pub fn post_new_admin(
    admin: AdminGuard,
    new: Json<NewAdmin>,
) -> QueryResult<Status>
{
    if select_admin_by_login(&new.login, &admin.connection).is_ok() {
        return Ok(Status::new(472, "login already taken"))
    }
    let new = new.into_inner();

    let (hash, salt) = PasswordHash::create(&new.password);

    let new_admin = Admin {
        first_name: new.first_name,
        last_name: new.last_name,
        login: Some(new.login),
        hash: Some(hash),
        salt: Some(salt),
        service: false,
        locked: false,
        owner: new.owner,
        expires: NaiveDate::from_yo(2019, 1),
    };

    new_admin.insert(&admin.connection)?;

    Ok(Status::new(204, "Success - No Content"))
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewUser
{
    name: String,
    password: String,
    card: Option<u64>,
    pin: Option<u32>,
}

#[post("/user", data = "<new>")]
pub fn post_new_user(
    admin: AdminGuard,
    new: Json<NewUser>,
) -> QueryResult<Status>
{
    let new = new.into_inner();
    match add_user(None, &new.name, &new.password, new.card, new.pin, false, &admin.connection) {
        Ok(()) => Ok(Status::new(204, "Success - No Content")),
        Err(UserAddError::UsernameExists) => Ok(Status::new(472, "username already taken")),
        Err(UserAddError::InsertError(e)) => Err(e),
    }
}
