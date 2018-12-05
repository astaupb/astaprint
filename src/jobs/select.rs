use chrono::NaiveDateTime;
/// AStAPrint - Jobs - select
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
use diesel::prelude::*;
use jobs::{
    data::JobData,
    table::*,
};

pub fn select_data_from_jobs(user_id: u32, connection: &MysqlConnection) -> QueryResult<Vec<JobData>>
{
    jobs::table
        .select((jobs::id, jobs::info, jobs::options, jobs::created))
        .filter(jobs::user_id.eq(user_id))
        .load(connection)
        .map(|result: Vec<(u32, Vec<u8>, Vec<u8>, NaiveDateTime)>| {
            result
                .iter()
                .map(|(id, info, options, created)| {
                    (JobData::from((*id, &info[..], &options[..], *created)))
                })
                .collect()
        })
}
