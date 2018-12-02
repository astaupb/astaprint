/// AStAPrint-Backend - User POST Routes
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
use rocket::response::status::Reset;

use rocket_contrib::Json;
use user::{
    guard::UserGuard,
    login::LoginGuard,
    tokens::table::*,
};

use diesel::{
    delete,
    prelude::*,
    QueryResult,
};

#[post("/tokens")]
pub fn login(login: LoginGuard) -> Json<String>
{
    Json(login.token)
}

#[post("/logout")]
pub fn logout(user: UserGuard) -> QueryResult<Reset>
{
    delete(user_tokens::table.filter(user_tokens::id.eq(user.token_id))).execute(&user.connection)?;

    info!("{} logged out", user.id);

    Ok(Reset)
}
