use rocket::response::Redirect;
/// AStAPrint-Backend - User GET Routes
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
use rocket_contrib::Json;

use bigdecimal::ToPrimitive;

use diesel::{
    prelude::*,
    QueryResult,
};

use journal::credit::get_credit;
use user::{
    guard::UserGuard,
    table::*,
    tokens::table::*,
};


#[derive(Serialize, Debug)]
struct UserInfo
{
    id: u32,
    name: String,
    credit: f64,
    tokens: usize,
    token_id: u32,
}

#[get("/")]
fn get_user_info(user: UserGuard) -> QueryResult<Json<UserInfo>>
{
    let id = user.id;
    let tokens: Vec<u32> = user_tokens::table
        .select(user_tokens::id)
        .filter(user_tokens::user_id.eq(id))
        .load(&user.connection)?;

    let tokens = tokens.len();

    let credit = get_credit(user.id, &user.connection)?.to_f64().unwrap();

    let name: String =
        user::table.select(user::name).filter(user::id.eq(user.id)).first(&user.connection)?;

    Ok(Json(UserInfo {
        id,
        name,
        credit,
        tokens,
        token_id: user.token_id,
    }))
}

#[get("/name")]
fn fetch_username(user: UserGuard) -> QueryResult<Json<String>>
{
    let username: String =
        user::table.select(user::name).filter(user::id.eq(user.id)).first(&user.connection)?;

    info!("{} fetched username", user.id);

    Ok(Json(username))
}

#[get("/credit")]
pub fn credit_redirect() -> Redirect
{
    Redirect::to("/astaprint/journal/credit")
}
