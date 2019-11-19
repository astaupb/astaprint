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

use diesel::QueryResult;

use crate::admin::guard::AdminGuard;

use mysql::printers::delete::*;

#[delete("/printers/<id>")]
pub fn delete_printer(admin: AdminGuard, id: u32) -> QueryResult<Status>
{
    delete_printer_by_device_id(id, &admin.connection)?;
    Ok(Status::new(205, "Reset Content"))
}
