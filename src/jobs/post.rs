// AStAPrint
// Copyright (C) 2019 AStA der Universität Paderborn
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

use diesel::QueryResult;

use rocket_contrib::json::Json;

use rocket::{
    State,
    http::Status,
};

use redis::share::Share;

use mysql::jobs::{
    select::*,
    insert::*,
};
use user::guard::UserGuard;

#[post("/sharecode", data = "<code>")]
pub fn post_sharecode(user: UserGuard, code: Json<String>, share: State<Share>) -> QueryResult<Status>
{
    if let Ok(id) = share.get(code.into_inner()) {
        if let Ok(job) = select_full_job_by_id(id, &user.connection) {
            insert_into_jobs(
                user.id,
                job.info,
                job.options,
                job.pdf,
                job.preview_0,
                job.preview_1,
                job.preview_2,
                job.preview_3,
                &user.connection,
            )?;

            Ok(Status::new(200, "OK"))
        } else {
            Ok(Status::new(404, "Job not found")) 
        }
    } else {
        Ok(Status::new(404, "Sharecode not found"))
    }
}
