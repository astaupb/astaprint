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
use rocket_contrib::json::Json;

use rocket::{
    http::Status,
    response::status::Custom,
    State,
};

use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use sodium::PasswordHash;

use model::{
    job::options::{
        update::{
            JobOptionsUpdate,
            Update,
        },
        JobOptions,
    },
    user::{
        PasswordChangeBody,
        UserSummary,
    },
};

use mysql::user::{
    delete::*,
    select::*,
    update::*,
};

use crate::user::{
    add::{
        add_user,
        NewUser,
        UserAddError::*,
    },
    guard::UserGuard,
};

#[get("/")]
pub fn get_user_summary(user: UserGuard) -> QueryResult<Json<UserSummary>>
{
    let id = user.id;
    let tokens: Vec<u32> = select_token_ids_of_user(user.id, &user.connection)?;

    let tokens = tokens.len();

    let info: (String, i32, Option<u64>, Option<u32>, Option<String>, bool) =
        select_user_info_by_id(user.id, &user.connection)?;

    Ok(Json(UserSummary {
        id,
        name: info.0,
        credit: info.1,
        card: info.2,
        pin: info.3,
        email: info.4,
        tou_accept: info.5,
        tokens,
        token_id: user.token_id,
    }))
}

#[post("/", data = "<user>")]
pub fn register_as_new_user(
    user: Json<NewUser>,
    pool: State<Pool<ConnectionManager<MysqlConnection>>>,
) -> QueryResult<Custom<()>>
{
    match add_user(user.into_inner(), &pool.get().expect("getting mysql connection")) {
        Ok(_id) => Ok(Custom(Status::new(204, "Success - No Content"), ())),
        Err(UsernameExists) => Ok(Custom(Status::new(470, "username already taken"), ())),
        Err(UsernameInvalid) => Ok(Custom(Status::new(471, "Invalid Username"), ())),
        Err(EmailExists) => Ok(Custom(Status::new(472, "email already taken"), ())),
        Err(QueryError(e)) => Err(e),
    }
}

#[get("/options")]
pub fn get_user_default_options(user: UserGuard) -> QueryResult<Json<JobOptions>>
{
    let options = match select_user_options(user.id, &user.connection)? {
        Some(options) => JobOptions::from(&options[..]),
        None => JobOptions::default(),
    };

    Ok(Json(options))
}

#[put("/options", data = "<update>")]
pub fn update_user_default_options(
    user: UserGuard,
    update: Json<JobOptionsUpdate>,
) -> QueryResult<Status>
{
    let mut options = if let Some(options) =
        select_user_options(user.id, &user.connection).expect("selecting user options")
    {
        JobOptions::from(&options[..])
    }
    else {
        JobOptions::default()
    };

    options.merge(update.into_inner());

    let value = Some(options.serialize());

    update_default_job_options(user.id, value, &user.connection)?;

    Ok(Status::new(205, "Success - Reset Content"))
}

#[put("/tou_accept?<tou_accept>")]
pub fn change_user_tou_accept(user: UserGuard, tou_accept: bool) -> Status
{
    match update_tou_accept_of_user(user.id, tou_accept, &user.connection) {
        Ok(1) => Status::new(205, "Success - Reset Content"),
        _ => Status::new(500, "Internal Server Error"),
    }
}

#[get("/name")]
pub fn fetch_username(user: UserGuard) -> QueryResult<Json<String>>
{
    let username: String = select_user_name_by_id(user.id, &user.connection)?.unwrap();

    info!("{} fetched username", user.id);

    Ok(Json(username))
}

#[put("/name", data = "<new_username>")]
pub fn change_username(user: UserGuard, new_username: Json<String>) -> QueryResult<Custom<()>>
{
    let name = new_username.into_inner();

    if name.chars().any(|c| !c.is_alphanumeric()) || name.bytes().count() > 32 {
        return Ok(Custom(Status::new(471, "Invalid Username"), ()))
    }

    let user_id = select_user_id_by_name_optional(&name, &user.connection)?;

    if user_id.is_some() {
        return Ok(Custom(Status::new(472, "Username Already Taken"), ()))
    }

    update_user_name(user.id, &name, &user.connection)?;

    info!("user#{} changed username", user.id);

    Ok(Custom(Status::new(205, "Reset Content"), ()))
}

#[put("/email?<email>")]
pub fn change_email(user: UserGuard, email: String) -> QueryResult<Custom<()>>
{
    let user_id = select_user_id_by_email_optional(&email, &user.connection)?;

    if user_id.is_some() {
        return Ok(Custom(Status::new(472, "Email Already Taken"), ()))
    }

    update_user_email(user.id, Some(email), &user.connection)?;

    Ok(Custom(Status::new(205, "Reset Content"), ()))
}

#[put("/password", data = "<body>")]
pub fn change_password(user: UserGuard, body: Json<PasswordChangeBody>) -> QueryResult<Status>
{
    let (hash, salt): (Vec<u8>, Vec<u8>) = select_hash_and_salt(user.id, &user.connection)?;

    if PasswordHash::with_salt(&body.password.old, &salt[..]) == hash {
        let (hash, salt) = PasswordHash::create(&body.password.new);

        update_hash_and_salt(user.id, hash, salt, &user.connection)?;

        delete_all_tokens_of_user(user.id, &user.connection)?;

        info!("{} changed password", user.id);

        Ok(Status::new(204, "No Content"))
    }
    else {
        info!("{} delivered wrong old password", user.id);

        Ok(Status::new(400, "Wrong Old Password"))
    }
}

#[get("/credit")]
pub fn fetch_credit(user: UserGuard) -> QueryResult<Json<i32>>
{
    Ok(Json(select_user_credit_by_id(user.id, &user.connection)?))
}

#[post("/logout")]
pub fn logout(user: UserGuard) -> QueryResult<Status>
{
    delete_user_token_by_id(user.id, user.token_id, &user.connection)?;

    info!("{} logged out", user.id);

    Ok(Status::new(205, "Reset View"))
}
