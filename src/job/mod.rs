/// AStAPrint-Common - Mod.rs
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
pub mod data;
pub mod files;
pub mod json;

pub use self::{data::{JobData,
                      ShortJobData},
               files::JobFiles,
               json::{DispatchWorkerJSON,
                      PrintWorkerJSON}};

#[derive(Serialize, Deserialize)]

pub struct Job
{
    pub data: JobData,
    pub files: JobFiles,
}

impl Job
{
    pub fn new(data: JobData) -> Job
    {
        let files = JobFiles::new(&data.uid, data.user_id);

        Job {
            data,
            files,
        }
    }
}
