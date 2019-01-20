/// AStAPrint
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

use diesel::{
    prelude::*,
    result::Error,
};

use sodium::PasswordHash;

use mysql::user::{
    insert::*,
    select::*,
};

#[derive(Debug)]
pub enum UserAddError
{
    UsernameExists,
    InsertError(Error),
}

impl From<Error> for UserAddError
{
    fn from(err: Error) -> UserAddError
    {
        UserAddError::InsertError(err)
    }
}

pub fn add_user(
    id: Option<u32>,
    name: &str,
    password: &str,
    card: Option<u64>,
    pin: Option<u32>,
    locked: bool,
    connection: &MysqlConnection,
) -> Result<(), UserAddError>
{
    let user_id: Option<u32> =
        select_user_id_by_name(name, connection)?;

    if user_id.is_some() {
        return Err(UserAddError::UsernameExists);
    }

    let (hash, salt) = PasswordHash::create(password);

    match id {
        Some(id) => {
            insert_into_user_with_id(id, name, hash, salt, card, pin, locked, connection)?;
        },
        None => {
            insert_into_user(name, hash, salt, card, pin, locked, connection)?;
        },
    }

    Ok(())
}
