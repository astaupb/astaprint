use bincode;
/// AStAPrint - Jobs
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
use chrono::NaiveDateTime;

use crate::jobs::data::{
    JobInfo,
    JobOptions,
};

pub mod data;
pub mod pdf;
pub mod post;
pub mod task;
pub mod tmp;
pub mod uid;

table! {
    jobs (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        info -> Binary,
        options -> Binary,
        data -> Longblob,
        preview_0 -> Mediumblob,
        preview_1 -> Nullable<Mediumblob>,
        preview_2 -> Nullable<Mediumblob>,
        preview_3 -> Nullable<Mediumblob>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
pub struct Job
{
    pub id: u32,
    pub user_id: u32,
    pub info: Vec<u8>,
    pub options: Vec<u8>,
    pub data: Vec<u8>,
    pub preview_0: Vec<u8>,
    pub preview_1: Option<Vec<u8>>,
    pub preview_2: Option<Vec<u8>>,
    pub preview_3: Option<Vec<u8>>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

impl Job
{
    pub fn info(&self) -> JobInfo
    {
        bincode::deserialize(&self.info[..]).expect("deserialzing JobInfo")
    }

    pub fn options(&self) -> JobOptions
    {
        bincode::deserialize(&self.info[..]).expect("deserializing JobOptions")
    }

    pub fn set_info(&mut self, info: JobInfo)
    {
        self.info = bincode::serialize(&info).expect("serializing JobInfo");
    }

    pub fn set_options(&mut self, options: JobOptions)
    {
        self.options = bincode::serialize(&options).expect("serializing JobOptions");
    }

    pub fn pages_to_print(&self) -> u16
    {
        let info = self.info();
        let options = self.options();
        let mut count = info.pagecount;

        count = (count / u16::from(options.nup))
            + match info.pagecount % u16::from(options.nup) {
                0 => 0,
                _ => 1,
            };

        if options.a3 {
            count *= 2;
        }

        count * options.copies
    }
}

#[test]
fn pages_to_print()
{
    let mut data = JobData::new("uid", 1, "filename", "password", true);
    data.info.pagecount = 18;
    data.options.nup = 4;
    println!("{:?}, pages to print: {}", data, data.pages_to_print());
}
