/// AStAPrint-Backend - User Token Routes
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
use diesel::{
    self,
    prelude::*,
};
use rocket::response::status::Reset;
use rocket_contrib::Json;

use crate::{
    guards::UserGuard,
    user::response::UserTokenResponse,
};

use crate::user::*;

#[get("/")]
pub fn get_all_tokens(user: UserGuard) -> Result<Json<Vec<UserTokenResponse>>, diesel::result::Error>
{
    let tokens: Vec<UserToken> = user_token::table
        .select(user_token::all_columns)
        .filter(user_token::user_id.eq(user.id))
        .load(&user.connection)?;

    info!("{} fetched all tokens", user.id);

    Ok(Json(tokens.iter().map(|row| UserTokenResponse::from(row)).collect()))
}

#[delete("/")]
pub fn delete_all_tokens(user: UserGuard) -> Result<Reset, diesel::result::Error>
{
    diesel::delete(user_token::table.filter(user_token::user_id.eq(user.id))).execute(&user.connection)?;

    info!("{} deleted all tokens", user.id);

    Ok(Reset)
}

#[delete("/<token_id>")]
pub fn delete_single_token(user: UserGuard, token_id: u32) -> Result<Option<Reset>, diesel::result::Error>
{
    let affected_rows = diesel::delete(
        user_token::table.filter(user_token::user_id.eq(user.id)).filter(user_token::id.eq(token_id)),
    )
    .execute(&user.connection)?;
    if affected_rows > 0 {
        info!("{} deleted token {}", user.id, &token_id);
        Ok(Some(Reset))
    } else {
        Ok(None)
    }
}

#[get("/<token_id>")]
pub fn get_single_token(
    user: UserGuard,
    token_id: u32,
) -> Result<Option<Json<UserTokenResponse>>, diesel::result::Error>
{
    let token: Option<UserToken> = user_token::table
        .select(user_token::all_columns)
        .filter(user_token::id.eq(token_id))
        .first(&user.connection)
        .optional()?;

    Ok(token.map(|x| Json(UserTokenResponse::from(&x))))
}
