/// AStAPrint - Examples - JournalPost
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
    journal::insert,
    pool::*,
};

use std::{
    env,
    str::FromStr,
};

fn main()
{
    let arg: Vec<_> = env::args().collect();
    if arg.len() != 4 {
        panic!("pass user_id, value, description");
    }
    let value = BigDecimal::from_str(&arg[2]).unwrap();
    let user_id: u32 = arg[1].parse().unwrap();
    let redis_pool = create_redis_pool(&env::var("ASTAPRINT_REDIS_URL").unwrap(), 3);
    let mysql_pool = create_mysql_pool(3);

    insert(user_id, value, &arg[3], redis_pool, mysql_pool.get().unwrap()).unwrap();
}
