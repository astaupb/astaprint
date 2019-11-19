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

use crate::printers::update::PrinterUpdate;

use mysql::printers::{
    select::*,
    update::*,
};

#[put("/printers/<id>", data = "<update>")]
pub fn put_printer_details(
    admin: AdminGuard,
    id: u32,
    update: Json<PrinterUpdate>,
) -> QueryResult<Status>
{
    let printer = update.into_inner().update(select_printer_by_device_id(id, &admin.connection)?);
    update_printer(printer, &admin.connection)?;
    Ok(Status::new(205, "Reset Content"))
}
