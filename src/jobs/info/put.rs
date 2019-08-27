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
use diesel::result::QueryResult;

use rocket_contrib::json::Json;
use rocket::http::Status;

use model::job::info::JobInfo;

use mysql::jobs::{
    select::*,
    update::*,
};

use user::guard::UserGuard;

#[put("/<id>/info/filename", data = "<filename>")]
pub fn update_filename(user: UserGuard, id: u32, filename: Json<String>) -> QueryResult<Status>
{
    let result: Option<Vec<u8>> = select_job_info(id, user.id, &user.connection)?;

    Ok(if let Some(value) = result {
        let mut info: JobInfo = bincode::deserialize(&value[..]).expect("deserializing JobInfo");
        info.filename = filename.into_inner();

        if update_job_info(id, user.id, bincode::serialize(&info).expect("serializing JobInfo"), &user.connection)? == 1 {
            Status::new(205, "Reset Content") 
        } else {
            Status::new(401, "Bad Request") 
        }
         
    } else {
        Status::new(404, "Job not found") 
    })
}
