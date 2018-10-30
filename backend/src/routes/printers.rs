/// AStAPrint-Backend - Printers Routes
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
use crate::{
    environment::Environment,
    guards::User,
};
use rocket::{
    response::status::Accepted,
    State,
};

use std::fs::copy;

#[derive(FromForm)]
pub struct QueuePostQuery
{
    uid: String,
}

#[post("/<device_id>/queue?<query>")]
pub fn print_job<'a>(
    user: User,
    env: State<Environment>,
    device_id: String,
    query: QueuePostQuery,
) -> Option<Accepted<&'a str>>
{
    let job_file = format!("{}/{}/index/{}", env.userdir, user.id, query.uid);

    let print_spool = format!("{}/print/{}/incoming/{}", env.spooldir, device_id, query.uid);

    if copy(&job_file, &print_spool).is_ok() {
        info!("{} posted {} to printer {}", user.id, &query.uid[..8], &device_id);

        Some(Accepted(Some("started processing")))
    } else {
        None
    }
}
