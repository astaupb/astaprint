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
use diesel::prelude::*;

use rocket_contrib::json::Json;

use mysql::printers::select::{
    select_printer_by_device_id,
    select_printers,
};

use model::printer::UserPrinterResponse;

use crate::user::guard::UserGuard;

#[get("/")]
pub fn get_printers(user: UserGuard) -> QueryResult<Json<Vec<UserPrinterResponse>>>
{
    Ok(Json(select_printers(&user.connection)?.iter().map(UserPrinterResponse::from).collect()))
}

#[get("/<device_id>")]
pub fn get_single_printer(user: UserGuard, device_id: u32)
-> QueryResult<Json<UserPrinterResponse>>
{
    Ok(Json(UserPrinterResponse::from(&select_printer_by_device_id(device_id, &user.connection)?)))
}
