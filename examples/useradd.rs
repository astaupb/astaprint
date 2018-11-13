/// AStAPrint - Examples - useradd
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
extern crate bigdecimal;
use bigdecimal::{
    BigDecimal,
    FromPrimitive,
};

extern crate diesel;
use diesel::{
    insert_into,
    prelude::*,
};

extern crate astacrypto;
use astacrypto::PasswordHash;

extern crate astaprint;
use astaprint::{
    journal::*,
    user::*,
};

use std::env;

fn main()
{
    let arg: Vec<_> = env::args().collect();
    if arg.len() != 3 {
        panic!("expecting username password");
    }
    let url = env::var("ASTAPRINT_DATABASE_URL").expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = MysqlConnection::establish(&url).expect("establishing MysqlConnection");

    let user_id: Option<u32> = user::table
        .select(user::id)
        .filter(user::name.eq(arg[1].clone()))
        .first(&connection)
        .optional()
        .expect("getting username");

    if user_id.is_some() {
        panic!("username already taken");
    }
    let (hash, salt) = PasswordHash::create(&arg[2]);
    insert_into(user::table)
        .values((user::name.eq(arg[1].clone()), user::hash.eq(hash), user::salt.eq(salt)))
        .execute(&connection)
        .expect("adding user");

    let user_id: u32 = user::table
        .select(user::id)
        .filter(user::name.eq(arg[1].clone()))
        .first(&connection)
        .expect("getting user id");

    insert_into(journal::table)
        .values((
            journal::user_id.eq(user_id),
            journal::value.eq(BigDecimal::from_u32(0).unwrap()),
            journal::description.eq("from example".to_string()),
        ))
        .execute(&connection)
        .expect("inserting into journal");
}
