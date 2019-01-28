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
use rocket::{
    http::Status,
};
use rocket_contrib::json::Json;

use user::guard::UserGuard;

use admin::guard::AdminGuard;

use legacy::tds::insert_transaction;


/*
pub fn insert_transaction(
    user_id: u32,
    value: BigDecimal,
    description: &str,
    without_money: bool,
    admin_id: Option<u32>,
)
*/

#[derive(Deserialize, Debug, Clone)]
pub struct JournalPost
{
    user_id: u32,
    value: i32,
    description: String,
    without_money: bool,
}

#[post("/journal", data = "<body>")]
pub fn post_to_journal_as_admin(body: Json<JournalPost>, admin: AdminGuard) -> Status {
    insert_transaction(
        body.user_id,
        body.value,
        &body.description,
        body.without_money,
        Some(admin.id),
    );
    Status::new(204, "Success - No Content")
}
