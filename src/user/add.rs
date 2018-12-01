/// AStAPrint
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use bigdecimal::BigDecimal;

use diesel::{
    insert_into,
    prelude::*,
    r2d2::{
        ConnectionManager,
        PooledConnection,
    },
};

use astacrypto::PasswordHash;

use user::*;

#[derive(Debug)]
pub enum UserAddError
{
    UsernameExists,
}

pub fn add_user(
    name: &str,
    password: &str,
    locked: bool,
    credit: BigDecimal,
    description: &str,
    connection: PooledConnection<ConnectionManager<MysqlConnection>>,
) -> Result<(), UserAddError>
{
    let user_id: Option<u32> = user::table
        .select(user::id)
        .filter(user::name.eq(name))
        .first(&connection)
        .optional()
        .expect("getting username");

    if user_id.is_some() {
        return Err(UserAddError::UsernameExists);
    }

    let (hash, salt) = PasswordHash::create(password);

    insert_into(user::table)
        .values((user::name.eq(name), user::hash.eq(hash), user::salt.eq(salt), user::locked.eq(locked)))
        .execute(&connection)
        .expect("adding user");

    let user_id: u32 = user::table
        .select(user::id)
        .filter(user::name.eq(name))
        .first(&connection)
        .expect("getting user id");

    insert_into(journal::table)
        .values((
            journal::user_id.eq(user_id),
            journal::value.eq(credit),
            journal::description.eq(description),
        ))
        .execute(&connection)
        .expect("inserting into journal");

    Ok(())
}
