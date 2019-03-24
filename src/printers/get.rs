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
use admin::guard::AdminGuard;
use diesel::prelude::*;
use mysql::printers::select::{
    select_printer_by_device_id,
    select_printers,
};
use printers::response::PrinterResponse;
use rocket_contrib::json::Json;

#[get("/printers")]
pub fn get_printers(admin: AdminGuard) -> QueryResult<Json<Vec<PrinterResponse>>>
{
    Ok(Json(select_printers(&admin.connection)?.iter().map(PrinterResponse::from).collect()))
}

#[get("/printers/<id>")]
pub fn get_single_printer(
    id: u32,
    admin: AdminGuard,
) -> QueryResult<Json<PrinterResponse>>
{
    let connection: &MysqlConnection = &admin.connection;
    Ok(Json(PrinterResponse::from(select_printer_by_device_id(id, connection)?)))
}
