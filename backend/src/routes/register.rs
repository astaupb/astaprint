/// AStAPrint-Backend - Register Route
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
use rocket::{
    http::Status,
    request::Request,
    response::status::{
        BadRequest,
        NoContent,
    },

    response::{
        Responder,
        Response,
    },
    State,
};
use rocket_contrib::Json;

use diesel::{
    self,
    insert_into,
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
    result::{
        DatabaseErrorKind::UniqueViolation,
        Error::DatabaseError,
    },
    update,
};

use bigdecimal::BigDecimal;
use std::{
    fs::create_dir,
    str::FromStr,
};

use astaprint::database::user::{
    representation::RegisterToken,
    schema::*,
};
use crate::{
    crypto::hash_password,
    environment::Environment,
};

#[derive(Deserialize, Debug)]
struct RegisterUser
{
    username: String,
    password: String,
    token: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterError
{
    UsernameTaken,
    InvalidUsername,
    InvalidToken,
    TokenAlreadyConsumed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegisterResponse(RegisterError);

impl<'r> Responder<'r> for RegisterResponse
{
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status>
    {
        Response::build()
            .status(match self.0 {
                RegisterError::UsernameTaken => Status::new(470, "Username Taken"),
                RegisterError::InvalidUsername => Status::new(471, "Invalid Username"),
                RegisterError::TokenAlreadyConsumed => Status::new(472, "Token Already Consumed"),
                RegisterError::InvalidToken => Status::new(473, "Invalid Token"),
            })
            .ok()
    }
}

#[post("/", data = "<user>")]

fn register<'a>(
    user: Json<RegisterUser>,
    env: State<Environment>,
    pool: State<Pool<ConnectionManager<MysqlConnection>>>,
) -> Result<Result<Result<NoContent, RegisterResponse>, BadRequest<&'a str>>, diesel::result::Error>
{
    let connection = pool.get().unwrap();

    if user.username.chars().any(|c| !c.is_alphanumeric()) || user.username.bytes().count() > 32 {
        return Ok(Ok(Err(RegisterResponse(RegisterError::InvalidUsername))));
    }

    let result: Option<RegisterToken> = register_token::table
        .select(register_token::all_columns)
        .filter(register_token::value.eq(&user.token))
        .first(&connection)
        .optional()?;

    let token = match result {
        Some(token) => token,
        None => return Ok(Ok(Err(RegisterResponse(RegisterError::InvalidToken)))),
    };

    if !token.used {
        let (hash, salt) = hash_password(&user.password);

        match insert_into(user::table)
            .values((
                user::name.eq(&user.username),
                user::locked.eq(false),
                user::password_hash.eq(hash),
                user::password_salt.eq(salt),
            ))
            .execute(&connection)
        {
            Err(err) => {
                if let DatabaseError(UniqueViolation, _) = err {
                    info!("sometried to register with already taken username {}", &user.username);

                    return Ok(Ok(Err(RegisterResponse(RegisterError::UsernameTaken))));
                } else {
                    return Err(err);
                }
            },
            Ok(_) => {
                let user_id: u32 = user::table
                    .select(user::id)
                    .filter(user::name.eq(&user.username))
                    .first(&connection)?;

                update(register_token::table.filter(register_token::value.eq(&user.token)))
                    .set((register_token::user_id.eq(user_id), register_token::used.eq(true)))
                    .execute(&connection)?;

                let credit = BigDecimal::from_str("3.00").unwrap();

                insert_into(journal::table)
                    .values((
                        journal::user_id.eq(user_id),
                        journal::value.eq(&credit),
                        journal::credit.eq(&credit),
                        journal::description.eq(String::from("registered with token")),
                    ))
                    .execute(&connection)?;

                let userdir = format!("{}/{}", env.userdir, user_id);

                create_dir(&userdir).expect("creating user directory");

                create_dir(&format!("{}/index", userdir)).expect("creating index directory");

                create_dir(&format!("{}/tmp", userdir)).expect("creating tmp directory");

                create_dir(&format!("{}/pdf", userdir)).expect("creating pdf directory");

                create_dir(&format!("{}/preview", userdir)).expect("creating preview directory");

                info!("{}#{} registered with token {}", &user.username, user_id, &user.token);

                Ok(Ok(Ok(NoContent)))
            },
        }
    } else {
        info!("{} tried to register with already used token {}", &user.username, &token.value);

        Ok(Ok(Err(RegisterResponse(RegisterError::TokenAlreadyConsumed))))
    }
}
