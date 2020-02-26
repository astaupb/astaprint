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

use diesel::prelude::QueryResult;

use admin::{
    guard::AdminGuard,
    admins::AdminResponse,
};
use rocket_contrib::json::Json;

use mysql::admin::select::{
    select_admin,
    select_admin_by_id,
};

#[get("/")]
pub fn get_admins(admin: AdminGuard) -> QueryResult<Json<Vec<AdminResponse>>>
{
    Ok(Json(select_admin(&admin.connection)?.iter().map(AdminResponse::from).collect()))
}

#[get("/<id>")]
pub fn get_single_admin(admin: AdminGuard, id: u32) -> QueryResult<Json<AdminResponse>>
{
    Ok(Json(AdminResponse::from(&select_admin_by_id(id, &admin.connection)?)))
}
