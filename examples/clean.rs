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
extern crate bincode;
use bincode::deserialize;

extern crate chrono;
use chrono::NaiveDateTime;

extern crate mysql;
use mysql::{
    create_mysql_pool,
    jobs::{
        delete::*,
        select::*,
    },
};

extern crate model;
use model::job::options::JobOptions;

use std::{
    env,
    time::{
        SystemTime,
        UNIX_EPOCH,
    },
};

fn main()
{
    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = create_mysql_pool(&mysql_url, 1).get().unwrap();

    let jobs: Vec<(u32, Vec<u8>, Vec<u8>, NaiveDateTime)> =
        select_jobs_essentials(&connection).expect("selecting essentials of all jobs");

    let mut cleaned = 0;

    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("getting timestamp").as_secs();

    for (id, _info, options, created) in jobs {
        if now as i64 - created.timestamp() > 60 * 60 * 24 * 3 {
            let options: JobOptions = deserialize(&options[..]).expect("deserializing JobOptions");
            if !options.keep {
                delete_job_by_id(id, &connection).expect("deleting job");
                cleaned += 1;
            }
        }
    }
    println!("{} jobs cleaned", cleaned);
}
