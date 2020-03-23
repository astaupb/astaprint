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
use rocket_contrib::json::Json;

use model::job::{
    info::JobInfo,
    options::{
        pagerange::PageRange,
        JobOptions,
    },
};

use jobs::options::{
    JobOptionsUpdate,
    Update,
};

use mysql::jobs::{
    select::*,
    update::*,
};
use user::guard::UserGuard;

#[put("/<id>/options", data = "<update>")]
pub fn update_options(
    user: UserGuard,
    id: u32,
    update: Json<JobOptionsUpdate>,
) -> QueryResult<Status>
{
    if let Some((id, info, options, _created, _updated)) =
        select_job_of_user(user.id, id, &user.connection)?
    {
        let info: JobInfo = JobInfo::from(&info[..]);
        let mut options = JobOptions::from(&options[..]);

        options.merge(update.into_inner());

        if let Some(range) = PageRange::new(&options.range, info.pagecount as usize) {
            options.range = format!("{}", range);
            let _char = options.range.pop();
        }
        else {
            options.range = String::from("");
        }

        update_job_options(id, user.id, options.serialize(), &user.connection)?;

        Ok(Status::new(205, "Reset Content"))
    }
    else {
        Ok(Status::new(404, "Not Found"))
    }
}
