/// AStAPrint-Backend - Jobs Response
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

use jobs::data::{JobOptions, JobInfo};
use chrono::NaiveDateTime;
use bincode;

#[derive(Serialize, Deserialize, Debug)]
pub struct JobResponse
{
    id: u32,
    user_id: u32,
    timestamp: i64,
    info: JobInfo,
    options: JobOptions,
}

impl From<(u32, u32, NaiveDateTime, Vec<u8>, Vec<u8>)> for JobResponse
{
    fn from(row: (u32, u32, NaiveDateTime, Vec<u8>, Vec<u8>)) -> JobResponse
    {
        JobResponse {
            id: row.0,
            user_id: row.1,
            timestamp: row.2.timestamp(),
            info: bincode::deserialize(&row.3[..])
                .expect("deserializing JobInfo"),
            options: bincode::deserialize(&row.4[..])
                .expect("deserializing JobOptions"),
        }
    }
}
