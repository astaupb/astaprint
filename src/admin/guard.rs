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

use chrono::{
    offset::Utc,
    NaiveDate,
};

use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
        PooledConnection,
    },
};

use sodium::GenericHash;

use crate::user::{
    key::split_x_api_key,
    login::parse_header,
};

use mysql::admin::{
    select::*,
    update::*,
};

/// request guard for admin accesses
pub struct AdminGuard
{
    pub id: u32,
    pub token_id: u32,
    pub service: bool,
    pub connection: PooledConnection<ConnectionManager<MysqlConnection>>,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminGuard, ()>
    {
        let key: Vec<_> = request.headers().get("x-api-key").collect();

        if key.len() != 1 {
            info!("invalid x-api-key header {:?}", key);
            return Outcome::Failure((Status::BadRequest, ()))
        }
        let buf: Vec<u8> = match base64::decode_config(key[0], base64::URL_SAFE) {
            Ok(buf) => buf,
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
        };

        if buf.len() != 132 {
            return Outcome::Failure((Status::BadRequest, ()))
        }
        let (admin_id, token) = match split_x_api_key(buf) {
            Ok((admin_id, token)) => (admin_id, token),
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
        };

        let pool = request.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;

        let connection = match pool.get() {
            Ok(connection) => connection,
            Err(_) => return Outcome::Failure((Status::InternalServerError, ())),
        };

        // select password hash of user which is used as salt
        let result: QueryResult<(Vec<u8>, NaiveDate, bool)> =
            select_admin_hash_and_expires_and_service_by_id(admin_id, &connection);

        if let Ok((salt, expires, service)) = result {
            if Utc::today().naive_utc() >= expires {
                info!("admin {} accesss expired", admin_id);
                return Outcome::Failure((Status::Unauthorized, ()))
            }

            let hash = GenericHash::with_salt(&token[..], &salt[..]);

            match select_admin_token_id_by_hash(admin_id, hash, &connection) {
                Ok(token_id) => {
                    // update token so we can track the last usage time
                    let (ip, mut location) =
                        select_admin_token_ip_and_location_by_id(token_id, &connection)
                            .expect("selecting admin token");

                    let (user_agent, ip, new_location) = parse_header(request, Some(ip))?;

                    if let Some(new_location) = new_location {
                        location = new_location;
                    }

                    update_admin_token(token_id, user_agent, ip, location, &connection)
                        .expect("updating admin token");

                    Outcome::Success(AdminGuard {
                        id: admin_id,
                        token_id,
                        service,
                        connection,
                    })
                },
                Err(e) => {
                    error!("{:?}", e);
                    Outcome::Failure((Status::Unauthorized, ()))
                },
            }
        }
        else {
            info!("could not find hash for user {}", admin_id);
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
