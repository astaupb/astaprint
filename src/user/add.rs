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
use bigdecimal::BigDecimal;

use diesel::prelude::*;
use r2d2_redis::{
    r2d2::Pool,
    RedisConnectionManager,
};

use sodium::PasswordHash;

use legacy::tds::insert_transaction;

use mysql::user::{
    insert::*,
    select::*,
};
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
    redis: Pool<RedisConnectionManager>,
    connection: &MysqlConnection,
) -> Result<(), UserAddError>
{
    let user_id: Option<u32> =
        select_user_id_by_name(name, connection).expect("getting username");

    if user_id.is_some() {
        return Err(UserAddError::UsernameExists);
    }

    let (hash, salt) = PasswordHash::create(password);

    insert_into_user(name, hash, salt, locked, connection)
        .expect("inserting user");

    let user_id: u32 = select_user_id_by_name(name, connection)
        .expect("selecting user id")
        .expect("id is some");

    insert_transaction(user_id, credit, description, redis);


    Ok(())
}
