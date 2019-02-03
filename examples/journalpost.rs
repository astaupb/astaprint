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
extern crate legacy;
use legacy::tds::insert_transaction;

extern crate redis;
use redis::{
    get_redis_pool,
    lock::Lock,
    Redis,
};

use std::env;

fn main()
{
    let arg: Vec<_> = env::args().collect();
    if arg.len() != 6 {
        panic!("pass user_id, value, description, without_money, admin_id");
    }
    let value: i32 = arg[2].parse().unwrap();
    let user_id: u32 = arg[1].parse().unwrap();
    let admin_id: u32 = arg[5].parse().unwrap();

    let _lock = Lock::new("journal", get_redis_pool(3, Redis::Lock));

    insert_transaction(user_id, value, &arg[3], &arg[4] != "0", Some(admin_id));
}
