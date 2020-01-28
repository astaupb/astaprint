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

impl From<PrinterPost> for PrinterInsert
{
    fn from(post: PrinterPost) -> PrinterInsert
    {
        PrinterInsert {
            hostname: post.hostname,
            ip: post.ip,
            community: post.community,
            mac: post.mac,
            device_id: post.device_id,
            location: post.location,
            has_a3: post.has_a3,
            coin_operated: post.coin_operated,
            description: post.description,
        }
    }
}

#[post("/printers", data = "<post>")]
pub fn post_printer(admin: AdminGuard, post: Json<PrinterPost>) -> QueryResult<Status>
{
    insert_into_printers(PrinterInsert::from(post.into_inner()), &admin.connection)?;

    Ok(Status::new(200, "OK"))
}
