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
    options::pagerange::PageRange,
};

use jobs::*;

use mysql::jobs::update::*;

use user::guard::UserGuard;

#[put("/<id>/options", data = "<options>")]
pub fn update_options(
    user: UserGuard,
    id: u32,
    options: Json<JobOptions>,
) -> QueryResult<Status>
{
    if let Some(info) = select_job_info(id, user.id, &user.connection)? {
        let info: JobInfo = bincode::deserialize(&info[..]).expect("deserializing JobInfo");
        let mut options: JobOptions = options.into_inner();
        debug!("to parse: ({:?}, {:?}", options.range, info.pagecount);
        if let Some(range) = PageRange::new(&options.range, info.pagecount as usize) {
            debug!("range: {:?}", range);

            options.range = format!("{}", range);
            let _char = options.range.pop();
            debug!("options.range: {:?}", options.range);

            let serialized = bincode::serialize(&options).expect("serializing JobOptions");
            update_job_options(id, user.id, serialized, &user.connection)?;

            Ok(Status::new(205, "Reset Content"))
        }
        else {
            Ok(Status::new(400, "Bad Request"))
        }
    }
    else {
        Ok(Status::new(404, "Not Found"))
    }
}
