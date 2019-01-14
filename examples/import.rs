/// AStAPrint - Examples - legacy import
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

use user::add::add_user;

extern crate mysql;
use mysql::create_mysql_pool;

extern crate redis;
use redis::create_redis_pool;
extern crate diesel;

use std::{
    env,
    fs::File,
    io::Read,
    str::FromStr,
};

fn main()
{
    let redis_url = env::var("ASTAPRINT_REDIS_URL")
        .expect("reading ASTAPRINT_REDIS_URL from environment");

    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading mysql url from environment");

    let mut file = File::open("dump.tsv").unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("reading dump to string");

    let mut user_list: Vec<&str> = contents.split("\r\n").collect();

    let mysql_pool = create_mysql_pool(&mysql_url, 10);
    let redis_pool = create_redis_pool(&redis_url, 10);

    let mut user_count = 0;
    let connection = mysql_pool.get().unwrap();

    while user_list.len() > 0 {
        let mut end = 32;
        if user_list.len() < 32 {
            end = user_list.len();
        }
        user_count += end;
        for user in user_list.drain(..end) {
            let split: Vec<&str> = user.split('\t').collect();
            if split.len() < 3 {
                break;
            }
            println!("parsing {}", split[0]);
            let card: u32 = split[0].parse().unwrap();
            let pin: u32 = match split[1].parse() {
                Ok(pin) => pin,
                Err(_) => break,
            };
            continue;
            use diesel::{
                prelude::*,
                update,
            };
            /*
            println!("{} {}", card, pin);
            use astaprint::user::table::*;
            update(user::table.filter(user::name.eq(split[0])))
                .set((user::card.eq(Some(card)), user::pin.eq(Some(pin))))
                .execute(&connection)
                .expect("updating user");
                */
            // let locked = split[2] == "1";
            // let connection = mysql_pool.get().unwrap();
            // match add_user(
            // split[0],
            // split[1],
            // locked,
            // BigDecimal::from_str("0.0").unwrap(),
            // "imported",
            // redis_pool.clone(),
            // connection,
            // ) {
            // Ok(_) => println!("{} {} imported..", split[0], split[1]),
            // Err(e) => println!("{}: {:?}", split[0], e),
            // }
        }
        println!("{} imported", user_count);
    }
}
