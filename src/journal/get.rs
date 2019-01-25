/// AStAPrint
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as
/// published by the Free Software Foundation, either version 3 of the
/// License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this program.  If not, see <https://www.gnu.org/licenses/>.
use rocket_contrib::json::Json;

use user::guard::UserGuard;
use admin::guard::AdminGuard;

use model::journal::Transaction;

use legacy::tds::{
    get_journal_of_user, get_journal,
};


#[get("/?<desc>&<offset>&<limit>")]
pub fn get_journal_as_user(desc: Option<bool>, offset: Option<i32>, limit: Option<u32>, user: UserGuard) -> Json<Vec<Transaction>>
{
    Json(get_journal_of_user(
        user.id,
        desc.unwrap_or(true),
        offset.unwrap_or(0),
        limit.unwrap_or(u32::from(u16::max_value()) * 2),
    ))
}

#[get("/journal?<desc>&<offset>&<limit>")]
pub fn get_journal_as_admin(desc: Option<bool>, offset: Option<i32>, limit: Option<u32>, _admin: AdminGuard) -> Json<Vec<Transaction>>
{
    Json(get_journal(
        desc.unwrap_or(true),
        offset.unwrap_or(0),
        limit.unwrap_or(100),
    ))
}

