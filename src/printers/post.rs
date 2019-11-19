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

use rocket::http::Status;

use rocket_contrib::json::Json;

use diesel::QueryResult;

use crate::admin::guard::AdminGuard;

use mysql::printers::insert::*;

#[derive(Deserialize, Debug, Clone)]
pub struct PrinterPost
{
    pub hostname: String,
    pub ip: String,
    pub community: String,
    pub mac: String,
    pub device_id: u32,
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
    pub description: String,
}

#[post("/printers", data = "<post>")]
pub fn post_printer(admin: AdminGuard, post: Json<PrinterPost>) -> QueryResult<Status>
{
    insert_into_printers(
        post.hostname.clone(),
        post.ip.clone(),
        post.community.clone(),
        post.mac.clone(),
        post.device_id,
        post.location.clone(),
        post.has_a3,
        post.coin_operated,
        post.description.clone(),
        &admin.connection,
    )?;

    Ok(Status::new(200, "OK"))
}
