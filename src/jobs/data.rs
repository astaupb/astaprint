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
use bincode::deserialize;

use chrono::NaiveDateTime;

use jobs::{
    info::JobInfo,
    options::JobOptions,
};

#[derive(Debug)]
pub struct JobData
{
    pub id: u32,
    pub info: JobInfo,
    pub options: JobOptions,
    pub created: NaiveDateTime,
}
impl JobData
{
    pub fn pages_to_print(&self) -> u16
    {
        let mut count = self.info.pagecount;

        count = (count / u16::from(self.options.nup))
            + match self.info.pagecount % u16::from(self.options.nup) {
                0 => 0,
                _ => 1,
            };

        if self.options.a3 {
            count *= 2;
        }

        count * self.options.copies
    }
}

impl<'a> From<(u32, &'a [u8], &'a [u8], NaiveDateTime)> for JobData
{
    fn from((id, info, options, created): (u32, &'a [u8], &'a [u8], NaiveDateTime)) -> JobData
    {
        JobData {
            id,
            info: deserialize(info).expect("deserializing JobInfo"),
            options: deserialize(options).expect("deserializing JobOptions"),
            created,
        }
    }
}
