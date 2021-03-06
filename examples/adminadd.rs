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

use std::env;

extern crate astaprint;
use astaprint::admin::admins::add::{
    add_admin,
    NewAdmin,
};

fn main()
{
    let arg: Vec<_> = env::args().collect();
    if arg.len() != 5 {
        panic!("expecting first_name last_name login password");
    }
    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = create_mysql_pool(&mysql_url, 1).get().unwrap();

    add_admin(
        NewAdmin {
            first_name: arg[1].clone(),
            last_name: arg[2].clone(),
            login: arg[3].clone(),
            password: arg[4].clone(),
            service: None,
            locked: None,
            expires: None,
        },
        None,
        &connection,
    )
    .expect("adding Admin");
}
