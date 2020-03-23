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
use diesel::QueryResult;
use rocket_contrib::json::Json;

use model::user::UserTokenResponse;
use user::guard::UserGuard;

use mysql::user::{
    select::*,
    UserToken,
};

#[get("/")]
pub fn get_all_tokens(user: UserGuard) -> QueryResult<Json<Vec<UserTokenResponse>>>
{
    let tokens: Vec<UserToken> = select_user_tokens_by_user_id(user.id, &user.connection)?;

    info!("{} fetched all tokens", user.id);

    Ok(Json(tokens.iter().map(UserTokenResponse::from).collect()))
}

#[get("/<token_id>")]
pub fn get_single_token(
    user: UserGuard,
    token_id: u32,
) -> QueryResult<Option<Json<UserTokenResponse>>>
{
    let token: Option<UserToken> =
        select_single_user_token_optional(user.id, token_id, &user.connection)?;
    Ok(token.map(|row| Json(UserTokenResponse::from(&row))))
}
