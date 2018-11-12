/// AStAPrint-Backend - Request Guards - User
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
use base64;

use rocket::{
    http::Status,
    request::{
        self,
        FromRequest,
    },
    Outcome,
    Request,
    State,
};

use diesel::{
    self,
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
        PooledConnection,
    },
};

use crate::user::*;

pub struct UserGuard
{
    pub id: u32,
    pub token_id: u32,
    pub connection: PooledConnection<ConnectionManager<MysqlConnection>>,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserGuard, ()>
    {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();

        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let buf: Vec<u8> = match base64::decode_config(keys[0], base64::URL_SAFE) {
            Ok(buf) => buf,
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
        };

        let pool = request.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;

        let connection = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
        };

        let result: Result<(u32, u32), diesel::result::Error> = user_token::table
            .select((user_token::user_id, user_token::id))
            .filter(user_token::hash.eq(buf))
            .first(&connection);

        if let Ok((id, token_id)) = result {
            Outcome::Success(UserGuard {
                id,
                token_id,
                connection,
            })
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
