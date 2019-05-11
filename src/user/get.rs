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
use rocket::response::Redirect;
use rocket_contrib::json::Json;

use diesel::QueryResult;

use user::guard::UserGuard;

use model::job::options::JobOptions;

use mysql::user::select::*;

#[derive(Serialize, Debug)]
pub struct UserInfo
{
    id: u32,
    name: String,
    credit: i32,
    card: Option<u64>,
    pin: Option<u32>,
    tokens: usize,
    token_id: u32,
}

#[get("/")]
pub fn get_user_info(user: UserGuard) -> QueryResult<Json<UserInfo>>
{
    let id = user.id;
    let tokens: Vec<u32> = select_token_ids_of_user(user.id, &user.connection)?;

    let tokens = tokens.len();

    let info: (String, i32, Option<u64>, Option<u32>) =
        select_user_info_by_id(user.id, &user.connection)?;

    Ok(Json(UserInfo {
        id,
        name: info.0,
        credit: info.1,
        card: info.2,
        pin: info.3,
        tokens,
        token_id: user.token_id,
    }))
}

#[get("/options")]
pub fn get_user_default_options(user: UserGuard) -> QueryResult<Json<JobOptions>>
{
    let options = match select_user_options(user.id, &user.connection)? {
        Some(options) => bincode::deserialize(&options[..]).expect("deserializing JobOptions"),
        None => JobOptions::default(),
    };

    Ok(Json(options))
}

#[get("/name")]
pub fn fetch_username(user: UserGuard) -> QueryResult<Json<String>>
{
    let username: String = select_user_name_by_id(user.id, &user.connection)?.unwrap();

    info!("{} fetched username", user.id);

    Ok(Json(username))
}

#[get("/credit")]
pub fn credit_redirect() -> Redirect { Redirect::to("/astaprint/journal/credit") }
