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

extern crate chrono;
use chrono::NaiveDate;

extern crate sodium;
use sodium::pwhash::PasswordHash;

use std::env;

extern crate astaprint;
use astaprint::admin::admins::AdminCreate;

fn main()
{
    let arg: Vec<_> = env::args().collect();
    if arg.len() != 5 {
        panic!("expecting first_name last_name login password");
    }
    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = create_mysql_pool(&mysql_url, 1).get().unwrap();

    let first_name = arg[1].clone();
    let last_name = arg[2].clone();

    let login = arg[3].clone();

    let (hash, salt) = PasswordHash::create(&arg[4]);

    let expires = NaiveDate::from_yo(2019, 1);

    let admin = AdminCreate {
        first_name,
        last_name,
        login,
        hash,
        salt,
        service: false,
        locked: false,
        created_by: None,
        expires,
    };

    admin.insert(&connection).expect("inserting admin into database");
}
