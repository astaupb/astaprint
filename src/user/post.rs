use rocket::{
    http::Status,
    State,
};
/// AStAPrint-Backend - User POST Routes
/// Copyright (C) 2018  AStA der Universität Paderborn
///
/// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU Affero General Public License as
/// published by the Free Software Foundation, either version 3 of the
/// License, or (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU Affero General Public License for more details.
///
/// You should have received a copy of the GNU Affero General Public
/// License along with this program.  If not, see <https://www.gnu.org/licenses/>.
use rocket_contrib::json::Json;

use user::{
    guard::UserGuard,
    login::LoginGuard,
};

use diesel::{
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

use sodium::pwhash::PasswordHash;

use mysql::user::{
    delete::*,
    insert::*,
    select::*,
};

use legacy::tds::insert_empty_credit;

#[post("/tokens")]
pub fn login(login: LoginGuard) -> Json<String>
{
    Json(login.token)
}

#[post("/logout")]
pub fn logout(user: UserGuard) -> QueryResult<String>
{
    delete_user_token_by_id(user.id, user.token_id, &user.connection)?;

    info!("{} logged out", user.id);

    Ok("logged out".into())
}

#[derive(Deserialize, Debug)]
pub struct RegisterUser
{
    username: String,
    password: String,
}

#[post("/", data = "<user>")]
pub fn register_new_user(
    user: Json<RegisterUser>,
    mysql_pool: State<Pool<ConnectionManager<MysqlConnection>>>,
) -> QueryResult<Status>
{
    let connection = mysql_pool.get().expect("getting mysql connection from pool");

    if user.username.chars().any(|c| !c.is_alphanumeric())
        || user.username.bytes().count() > 32
    {
        return Ok(Status::new(471, "Invalid Username"));
    }

    let (hash, salt) = PasswordHash::create(&user.password);

    match insert_into_user(
        &user.username,
        hash,
        salt,
        None,
        None,
        false,
        &connection,
    ) {
        Err(err) => {
            if let DatabaseError(UniqueViolation, _) = err {
                info!(
                    "sometried to register with already taken username {}",
                    &user.username
                );
                return Ok(Status::new(470, "Username Taken"));
            } else {
                return Err(err);
            }
        },
        Ok(_) => {
            let user_id = select_user_id_by_name(&user.username, &connection)?.unwrap();

            insert_empty_credit(user_id);

            info!("{} registered with id {}", &user.username, user_id);

            Ok(Status::new(204, "Success - No Content"))
        },
    }
}
