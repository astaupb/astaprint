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
};
use r2d2_redis::RedisConnectionManager;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use std::str::FromStr;

use astacrypto::pwhash::PasswordHash;

use journal::insert;
use user::table::*;

table! {
    register_token (id) {
        id -> Unsigned<Smallint>,
        value -> Varchar,
        used -> Bool,
        user_id -> Nullable<Unsigned<Integer>>,
        created -> Timestamp,
    }
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "register_token"]
struct RegisterToken
{
    id: u16,
    value: String,
    used: bool,
    user_id: Option<u32>,
    created: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
struct RegisterUser
{
    username: String,
    password: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterError
{
    UsernameTaken,
    InvalidUsername,
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
            })
            .ok()
    }
}

#[post("/", data = "<user>")]
fn register<'a>(
    user: Json<RegisterUser>,
    mysql_pool: State<Pool<ConnectionManager<MysqlConnection>>>,
    redis_pool: State<Pool<RedisConnectionManager>>,
) -> QueryResult<Result<Result<NoContent, RegisterResponse>, BadRequest<&'a str>>>
{
    let connection = mysql_pool.get().expect("getting mysql connection from pool");

    if user.username.chars().any(|c| !c.is_alphanumeric()) || user.username.bytes().count() > 32 {
        return Ok(Ok(Err(RegisterResponse(RegisterError::InvalidUsername))));
    }

    let (hash, salt) = PasswordHash::create(&user.password);

    match insert_into(user::table)
        .values((
            user::name.eq(&user.username),
            user::locked.eq(false),
            user::hash.eq(hash),
            user::salt.eq(salt),
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
            let user_id: u32 =
                user::table.select(user::id).filter(user::name.eq(&user.username)).first(&connection)?;

            let credit = BigDecimal::from_str("0.00").unwrap();

            insert(user_id, credit, "registerd in beta", redis_pool.inner().clone(), connection)?;

            info!("{}#{} registered", &user.username, user_id);

            Ok(Ok(Ok(NoContent)))
        },
    }
}
