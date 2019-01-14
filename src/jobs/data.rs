/// AStAPrint Jobs - options.rs
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

use jobs::{
    info::JobInfo,
    options::JobOptions,
};

use bincode;

#[derive(Debug)]
pub struct JobData
{
    pub id: u32,
    pub info: JobInfo,
    pub options: JobOptions,
    pub created: NaiveDateTime,
}

impl From<(u32, Vec<u32>, Vec<u32>, NaiveDateTime)> for JobData
{
    fn from(data: (u32, Vec<u8>, Vec<u8>, NaiveDateTime)) -> JobData
    {
        JobData {
            id: data.0,
            info: bincode::deserialize(&data.1[..])
                .expect("deserializing JobInfo"),
            options: bincode::deserialze(&data.2[..])
                .expect("deserializing JobOptions"),
            created: data.3,
        } 
    }
}


