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

use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use sodium::{
    random_bytes,
    GenericHash,
    PasswordHash,
};

use mysql::admin::{
    insert::*,
    select::*,
    Admin,
};

use crate::user::{
    key::merge_x_api_key,
    login::parse_header,
};

/// request guard for admin login
pub struct AdminLoginGuard
{
    pub token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminLoginGuard
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminLoginGuard, ()>
    {
        let header = request.headers();

        let user_agent: Vec<_> = header.get("user-agent").collect();

        if user_agent.is_empty() {
            return Outcome::Failure((Status::BadRequest, ()))
        }
        let header: Vec<_> = header.get("authorization").collect();

        if header.is_empty() {
            return Outcome::Failure((Status::BadRequest, ()))
        }

        let auth: Vec<&str> = header[0].split(' ').collect();

        if auth.len() != 2 && auth[0] != "Basic" {
            return Outcome::Failure((Status::BadRequest, ()))
        }

        let decoded: Vec<u8> = match base64::decode_config(auth[1], base64::URL_SAFE) {
            Ok(decoded) => decoded,
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
        };

        let credentials = match String::from_utf8(decoded) {
            Ok(credentials) => credentials,
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
        };
        let credentials: Vec<&str> = credentials.split(':').collect();
        // expecting {username}:{password}
        if credentials.len() != 2 {
            return Outcome::Failure((Status::BadRequest, ()))
        }

        let pool = request.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;

        let connection = pool.get().expect("retrieving connection from pool");

        let admin: Admin = match select_admin_by_login(credentials[0], &connection) {
            Ok(admin) => admin,
            Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
        };
        debug!("{:?}", admin);
        if admin.locked {
            return Outcome::Failure((Status::Unauthorized, ()))
        }

        let (hash, salt) = (admin.hash, admin.salt);

        if PasswordHash::with_salt(credentials[1], &salt[..]) != hash {
            return Outcome::Failure((Status::Unauthorized, ()))
        }

        // generate token
        let token = random_bytes(128);

        // using the password hash as salt for performace reasons
        // and so every token gets invalidated on password change
        let hash = GenericHash::with_salt(&token[..], &hash[..]);

        let x_api_key = match merge_x_api_key(admin.id, token) {
            Ok(key) => key,
            Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
        };

        debug!("{:?}", x_api_key);

        // encode for client
        let x_api_key = base64::encode_config(&x_api_key[..], base64::URL_SAFE);

        let (user_agent, ip, location) = parse_header(request, None)?;

        match insert_admin_token((admin.id, user_agent, ip, location.unwrap(), hash), &connection) {
            Ok(_) => {
                info!("{} logged in ", admin.id);
                Outcome::Success(AdminLoginGuard {
                    token: x_api_key,
                })
            },
            Err(_) => {
                error!("failed to insert token into database");
                Outcome::Failure((Status::InternalServerError, ()))
            },
        }
    }
}
