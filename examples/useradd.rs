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
use bigdecimal::BigDecimal;

extern crate astaprint;
use astaprint::{
    user::add::add_user,
};

extern crate mysql;
use mysql::create_mysql_pool;

extern crate redis;
use redis::create_redis_pool;

use std::{
    env,
    str::FromStr,
};

fn main()
{
    let arg: Vec<_> = env::args().collect();
    if arg.len() != 3 {
        panic!("expecting username password");
    }
    let redis_url = env::var("ASTAPRINT_REDIS_URL")
        .expect("reading ASTAPRINT_REDIS_URL from environment");

    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = create_mysql_pool(&mysql_url, 1).get().unwrap();

    add_user(
        &arg[1],
        &arg[2],
        false,
        BigDecimal::from_str("0.0").unwrap(),
        "created from example/useradd.rs",
        create_redis_pool(&redis_url, 1),
        &connection,
    )
    .expect("adding user");
}
