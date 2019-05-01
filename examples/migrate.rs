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
    user::{
        select::*,
        update::*,
    },
};
extern crate astaprint;

extern crate model;
use model::job::{
    options::JobOptions,
    info::JobInfo,
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
        JobOptions{
            color: old.color,
            duplex: old.duplex,
            copies: old.copies,
            collate: old.collate,
            bypass: false,
            keep: old.keep,
            a3: old.a3,
            nup: old.nup,
            nuppageorder: old.nuppageorder,
            range: old.range.clone(),
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
        JobInfo{
            filename: old.filename.clone(),
            title: old.title.clone(),
            pagecount: old.pagecount,
            colored: old.colored,
            a3: old.a3,
        } 
    }
}

fn main()
{
    let arg: Vec<_> = env::args().collect();

    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = create_mysql_pool(&mysql_url, 1).get().unwrap();

    let job_ids = select_job_ids(&connection)
        .expect("selecting ids of all jobs");

    if arg.len() == 1 {
        if let Ok(options) = bincode::deserialize::<JobOptions>(&select_job_options_by_id(job_ids[0], &connection).expect("selecting JobOptions")) {
            println!("options: {:?}", options);
        };
            
    }
    if arg.len() == 2 {
        if arg[1] == "options" {
            for id in job_ids {
                let options: OldJobOptions = bincode::deserialize(
                    &select_job_options_by_id(id, &connection).expect("selecting JobOptions")
                ).expect("deserializing OldJobOptions");

                let value = bincode::serialize(&JobOptions::from(&options))
                    .expect("serializing JobOptions");

                update_job_options_by_id(id, value, &connection)
                    .expect("updating job options");
            }  
            println!("jobs migrated");

            // update user default options
            let user_ids = select_user_id(&connection)
                .expect("selecting user ids");

            let mut count = 0;
            for id in user_ids {
                let options: OldJobOptions = if let Some(value) = select_user_options(id, &connection).expect("selecting JobOptions") {
                    bincode::deserialize(&value).expect("deserializing OldJobOptions")
                } else {
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
        }
    
    }
}