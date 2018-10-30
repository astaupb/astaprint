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

use base64;

use crate::{
    guards::User,
    response::TokenResponse,
};

use astaprint::database::user::{
    representation::*,
    schema::*,
};

#[get("/")]
pub fn get_all_tokens(user: User) -> Result<Json<Vec<TokenResponse>>, diesel::result::Error>
{
    let tokens: Vec<Token> = token::table
        .select(token::all_columns)
        .filter(token::user_id.eq(user.id))
        .load(&user.connection)?;

    info!("{} fetched all tokens", user.id);

    Ok(Json(tokens.iter().map(|row| TokenResponse::from(row)).collect()))
}

#[delete("/")]
pub fn delete_all_tokens(user: User) -> Result<Reset, diesel::result::Error>
{
    diesel::delete(token::table.filter(token::user_id.eq(user.id))).execute(&user.connection)?;

    info!("{} deleted all tokens", user.id);

    Ok(Reset)
}

#[delete("/<short_token>")]
pub fn delete_single_token(user: User, short_token: String) -> Result<Option<Reset>, diesel::result::Error>
{
    match base64::decode_config(&short_token, base64::URL_SAFE) {
        Ok(binary_short_token) => {
            let tokens: Vec<Token> = token::table
                .select(token::all_columns)
                .load(&user.connection)?;

            for token in tokens.iter() {
                if &token.value[..6] == &binary_short_token[..] {
                    diesel::delete(
                        token::table.filter(token::user_id.eq(user.id)).filter(token::value.eq(&token.value)),
                    )
                    .execute(&user.connection)?;
                    info!("{} deleted token {}", user.id, &token.id);
                    return Ok(Some(Reset));
                }
            }
            return Ok(None);
        },
        Err(_) => Ok(None),
    }
}

#[get("/<short_token>")]
pub fn get_single_token(
    user: User,
    short_token: String,
) -> Result<Option<Json<TokenResponse>>, diesel::result::Error>
{
    match base64::decode_config(&short_token, base64::URL_SAFE) {
        Ok(binary_short_token) => {
            let tokens: Vec<Token> = token::table
                .select(token::all_columns)
                .load(&user.connection)?;

            for token in tokens.iter() {
                if &token.value[..6] == &binary_short_token[..] {
                    info!("{} fetched token info {}", user.id, token.id);
                    if token.user_id == user.id {
                        return Ok(Some(Json(TokenResponse::from(token))));
                    }
                }
            }
            return Ok(None);
        },
        Err(_) => Ok(None),
    }
}
