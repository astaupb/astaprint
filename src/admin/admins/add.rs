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

use chrono::{
    offset::Utc,
    Duration,
    NaiveDateTime,
};
use diesel::{
    prelude::*,
    result::Error,
};
use mysql::admin::{
    insert::insert_admin,
    select::select_admin_id_by_login_optional,
};

use sodium::PasswordHash;

#[derive(Deserialize, Debug, Clone)]
pub struct NewAdmin
{
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub password: String,
    pub service: Option<bool>,
    pub locked: Option<bool>,
    pub expires: Option<i64>,
}

#[derive(Debug)]
pub enum AdminAddError
{
    LoginExists,
    LoginInvalid,
    QueryError(Error),
}

impl From<Error> for AdminAddError
{
    fn from(err: Error) -> AdminAddError { AdminAddError::QueryError(err) }
}

pub fn add_admin(
    admin: NewAdmin,
    created_by: Option<u32>,
    connection: &MysqlConnection,
) -> Result<usize, AdminAddError>
{
    if admin.login.chars().any(|c| !c.is_alphanumeric()) || admin.login.bytes().count() > 32 {
        return Err(AdminAddError::LoginInvalid)
    }

    if select_admin_id_by_login_optional(&admin.login, connection)?.is_some() {
        return Err(AdminAddError::LoginExists)
    }

    let (hash, salt) = PasswordHash::create(&admin.password);

    let expires = if let Some(expires) = admin.expires {
        NaiveDateTime::from_timestamp(expires, 0 /* ns */).date()
    }
    else {
        Utc::today().naive_utc() + Duration::days(2 * 356)
    };

    Ok(insert_admin(
        (
            admin.first_name,
            admin.last_name,
            admin.login,
            hash,
            salt,
            admin.service.unwrap_or(true),
            admin.locked.unwrap_or(false),
            expires,
            created_by,
        ),
        connection,
    )?)
}
