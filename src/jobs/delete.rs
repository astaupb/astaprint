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

use rocket::http::Status;

use mysql::jobs::delete::*;
use user::guard::UserGuard;

#[delete("/<id>")]
pub fn delete_job(user: UserGuard, id: u32) -> QueryResult<Option<Status>>
{
    let deleted = delete_job_of_user_by_id(user.id, id, &user.connection)?;

    Ok(if deleted == 1 {
        Some(Status::new(205, "Reset Content"))
    }
    else {
        None
    })
}

#[delete("/")]
pub fn delete_all_jobs(user: UserGuard) -> QueryResult<Status>
{
    let _deleted = delete_all_jobs_of_user(user.id, &user.connection)?;

    Ok(Status::new(205, "Reset Content"))
}
