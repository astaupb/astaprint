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
extern crate mysql;
use mysql::create_mysql_pool;

extern crate astaprint;
use astaprint::user::add::{
    add_user,
    NewUser,
};

use std::env;

fn main()
{
    let arg: Vec<_> = env::args().collect();
    if arg.len() != 3 {
        panic!("expecting username password");
    }
    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = create_mysql_pool(&mysql_url, 1).get().unwrap();

    add_user(
        NewUser {
            name: arg[1].clone(),
            password: arg[2].clone(),
            email: None,
            locked: None,
        },
        &connection,
    )
    .expect("adding user");
}
