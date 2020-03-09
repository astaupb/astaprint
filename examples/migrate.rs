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
#[macro_use]
extern crate serde_derive;
extern crate mysql;
use mysql::{
    create_mysql_pool,
    jobs::{
        select::*,
        update::*,
    },
    journal::{
        select::*,
        update::*,
    },
    user::{
        select::*,
        update::*,
    },
};
extern crate astaprint;

extern crate model;
use model::job::{
    info::JobInfo,
    options::JobOptions,
};

extern crate bincode;

use std::env;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct OldJobOptions
{
    pub color: bool,
    pub duplex: u8,
    pub copies: u16,
    pub collate: bool,
    pub bypass: bool,
    pub keep: bool,
    pub a3: bool,
    pub nup: u8,
    pub nuppageorder: u8,
    pub range: String,
}

impl<'a> From<&'a OldJobOptions> for JobOptions
{
    fn from(old: &OldJobOptions) -> JobOptions
    {
        JobOptions {
            color: old.color,
            duplex: old.duplex,
            copies: old.copies,
            collate: old.collate,
            bypass: old.bypass,
            keep: old.keep,
            a3: old.a3,
            nup: old.nup,
            nuppageorder: old.nuppageorder,
            range: old.range.clone(),
            displayname: String::new(),
        }
    }
}

impl Default for OldJobOptions
{
    fn default() -> OldJobOptions
    {
        OldJobOptions {
            color: false,
            duplex: 0,
            copies: 1,
            collate: false,
            bypass: false,
            keep: false,
            a3: false,
            nup: 1,
            nuppageorder: 0,
            range: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OldJobInfo
{
    pub filename: String,
    pub title: String,
    pub pagecount: u32,
    pub colored: u32,
    pub a3: bool,
}

impl<'a> From<&'a OldJobInfo> for JobInfo
{
    fn from(old: &OldJobInfo) -> JobInfo
    {
        JobInfo {
            filename: old.filename.clone(),
            title: old.title.clone(),
            pagecount: old.pagecount,
            colored: old.colored,
            a3: old.a3,
            landscape: false,
        }
    }
}

fn main()
{
    let arg: Vec<_> = env::args().collect();

    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = create_mysql_pool(&mysql_url, 1).get().unwrap();

    let job_ids = select_job_ids(&connection).expect("selecting ids of all jobs");

    if arg.len() == 1 {
        if let Ok(options) = bincode::deserialize::<JobOptions>(
            &select_job_options_by_id(job_ids[0], &connection).expect("selecting JobOptions"),
        ) {
            println!("options: {:?}", options);
        }
        if let Ok(info) = bincode::deserialize::<JobInfo>(
            &select_job_info_by_id(job_ids[0], &connection).expect("selecting JobInfo"),
        ) {
            println!("info: {:?}", info);
        };
    }
    if arg.len() == 2 {
        if arg[1] == "options" {
            let mut count = 0;
            for id in job_ids {
                let options: OldJobOptions = bincode::deserialize(
                    &select_job_options_by_id(id, &connection).expect("selecting JobOptions"),
                )
                .expect("deserializing OldJobOptions");

                let value = bincode::serialize(&JobOptions::from(&options))
                    .expect("serializing JobOptions");

                update_job_options_by_id(id, value, &connection).expect("updating job options");
                count += 1;
                if count % 100 == 0 {
                    println!("job {} migrated", id);
                    count = 0;
                }
            }
            println!("jobs migrated");

            // update user default options
            let user_ids = select_user_id(&connection).expect("selecting user ids");

            for id in user_ids {
                let options: OldJobOptions = if let Some(value) =
                    select_user_options(id, &connection).expect("selecting JobOptions")
                {
                    bincode::deserialize(&value).expect("deserializing OldJobOptions")
                }
                else {
                    OldJobOptions::default()
                };

                let value = bincode::serialize(&JobOptions::from(&options))
                    .expect("serializing JobOptions");

                update_default_job_options(id, Some(value), &connection)
                    .expect("updating user default options");

                count += 1;
                if count % 100 == 0 {
                    println!("user {} migrated", id);
                    count = 0;
                }
            }
            println!("user migrated");
            // update options in print_journal
            let print_journal = select_print_journal(&connection).expect("selecting print_journal");
            for entry in print_journal {
                let options = bincode::deserialize::<OldJobOptions>(&entry.options)
                    .expect("deserializing JobOptions");
                update_print_journal_options_by_id(
                    entry.id,
                    bincode::serialize(&JobOptions::from(&options))
                        .expect("deserialzing JobOptions"),
                    &connection,
                )
                .expect("updating JobOptions");

                count += 1;
                if count % 100 == 0 {
                    println!("journal_entry {} migrated", entry.id);
                    count = 0;
                }
            }
            println!("options in journal migrated");
        }
        else if arg[1] == "info" {
            for id in job_ids {
                let info: OldJobInfo = bincode::deserialize(
                    &select_job_info_by_id(id, &connection).expect("selecting JobInfo"),
                )
                .expect("deserializing OldJobInfo");

                let value = bincode::serialize(&JobInfo::from(&info)).expect("serializing JobInfo");

                update_job_info_by_id(id, value, &connection).expect("updating job info");
            }
            println!("jobs migrated");
        }
    }
}
