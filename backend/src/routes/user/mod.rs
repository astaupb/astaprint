/// AStAPrint-Backend - User Routes
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
use rocket::response::status::{
    BadRequest,
    NoContent,
    Reset,
};
use rocket_contrib::Json;

use bigdecimal::{
    BigDecimal,
    ToPrimitive,
};

use diesel::{
    self,
    delete,
    prelude::*,
    update,
};

use crate::{
    crypto::{
        hash_password,
        verify_password,
    },
    guards::{
        Login,
        User,
    },
};

use astaprint::database::user::schema::*;

pub mod tokens;

#[derive(Deserialize, Debug)]
struct PasswordChange
{
    old: String,
    new: String,
}

#[derive(Deserialize, Debug)]
struct PasswordChangeBody
{
    password: PasswordChange,
}

#[derive(Serialize, Debug)]
struct UserInfo
{
    id: u32,
    name: String,
    credit: f64,
    tokens: usize,
}

#[get("/")]
fn get_user_info(user: User) -> Result<Json<UserInfo>, diesel::result::Error>
{
    let id = user.id;
    let tokens: Vec<u32> =
        token::table.select(token::id).filter(token::user_id.eq(id)).load(&user.connection)?;

    let tokens = tokens.len();

    let credit: BigDecimal = user::table
        .inner_join(journal::table)
        .select(journal::credit)
        .filter(user::id.eq(journal::user_id))
        .filter(user::id.eq(user.id))
        .order(journal::id.desc())
        .first(&user.connection)?;

    let credit = credit.to_f64().unwrap();

    let name: String =
        user::table.select(user::name).filter(user::id.eq(user.id)).first(&user.connection)?;

    Ok(Json(UserInfo {
        id,
        name,
        credit,
        tokens,
    }))
}


#[put("/password", data = "<body>")]
fn change_password(
    user: User,
    body: Json<PasswordChangeBody>,
) -> Result<Result<NoContent, BadRequest<&'static str>>, diesel::result::Error>
{
    let (hash, salt): (Vec<u8>, Vec<u8>) = user::table
        .select((user::password_hash, user::password_salt))
        .filter(user::id.eq(user.id))
        .first(&user.connection)?;

    if verify_password(&body.password.old, &hash[..], &salt[..]) {
        let (hash, salt) = hash_password(&body.password.new);

        update(user::table.filter(user::id.eq(user.id)))
            .set((user::password_hash.eq(hash), user::password_salt.eq(salt)))
            .execute(&user.connection)?;

        info!("{} changed password", user.id);

        Ok(Ok(NoContent))
    } else {
        info!("{} delivered wrong old password", user.id);

        Ok(Err(BadRequest(Some("wrong old password"))))
    }
}

#[get("/username")]
fn fetch_username(user: User) -> Result<Json<String>, diesel::result::Error>
{
    let username: String =
        user::table.select(user::name).filter(user::id.eq(user.id)).first(&user.connection)?;

    info!("{} fetched username", user.id);

    Ok(Json(username))
}

#[put("/username", data = "<new_username>")]
fn change_username(user: User, new_username: Json<String>) -> Result<Reset, diesel::result::Error>
{
    update(user::table.filter(user::id.eq(user.id)))
        .set(user::name.eq(new_username.into_inner()))
        .execute(&user.connection)?;

    info!("{} changed username", user.id);

    Ok(Reset)
}

#[get("/credit")]
pub fn credit(user: User) -> Result<Json<f64>, diesel::result::Error>
{
    let credit: BigDecimal = user::table
        .inner_join(journal::table)
        .select(journal::credit)
        .filter(user::id.eq(journal::user_id))
        .filter(user::id.eq(user.id))
        .order(journal::id.desc())
        .first(&user.connection)?;

    info!("{} fetched credit", user.id);

    Ok(Json(credit.to_f64().unwrap()))
}

#[post("/login")]
pub fn login(login: Login) -> String
{
    login.token
}

#[post("/logout")]
pub fn logout(user: User) -> Result<Reset, diesel::result::Error>
{
    delete(token::table.filter(token::id.eq(user.token_id))).execute(&user.connection)?;

    info!("{} logged out", user.id);

    Ok(Reset)
}
