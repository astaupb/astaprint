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
use diesel::{
    prelude::*,
    result::Error,
};

use sodium::PasswordHash;

use mysql::{
    insert_user,
    user::select::*,
};

#[derive(Deserialize, Debug, Clone)]
pub struct NewUser
{
    pub name: String,
    pub password: String,
    pub email: Option<String>,
    pub locked: Option<bool>,
}

#[derive(Debug)]
pub enum UserAddError
{
    UsernameExists,
    UsernameInvalid,
    EmailExists,
    QueryError(Error),
}

impl From<Error> for UserAddError
{
    fn from(err: Error) -> UserAddError { UserAddError::QueryError(err) }
}

pub fn add_user(user: NewUser, connection: &MysqlConnection) -> Result<u32, UserAddError>
{
    if user.name.chars().any(|c| !c.is_alphanumeric()) || user.name.bytes().count() > 32 {
        return Err(UserAddError::UsernameInvalid)
    }

    if select_user_id_by_name_optional(&user.name, connection)?.is_some() {
        return Err(UserAddError::UsernameExists)
    }

    if let Some(email) = &user.email {
        if select_user_id_by_email_optional(email, connection)?.is_some() {
            return Err(UserAddError::EmailExists)
        }
    }

    let (hash, salt) = PasswordHash::create(&user.password);

    Ok(insert_user(&user.name, hash, salt, user.email, user.locked.unwrap_or(false), connection)?)
}
