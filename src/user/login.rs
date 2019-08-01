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

use crate::user::key::merge_x_api_key;

use sodium::{
    random_bytes,
    GenericHash,
    PasswordHash,
};

use maxminddb::geoip2;

use mysql::user::{
    insert::*,
    select::*,
    User,
};

use std::net::{
    IpAddr,
    Ipv4Addr,
};

pub fn parse_header(request: &Request, old_ip: Option<String>) -> request::Outcome<(String, String, Option<String>), ()>
{
    let headers = request.headers();

    let user_agent: Vec<_> = headers.get("user-agent").collect();

    let user_agent = if user_agent[0].len() > 128 {
        String::from(&user_agent[0][.. 128])
    }
    else {
        String::from(user_agent[0])
    };

    let header: Vec<_> = headers.get("x-real-ip").collect();

    let ip: IpAddr = if header.is_empty() {
        "::1"
    }
    else {
        header[0]
    }
    .parse()
    .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let ip_str = format!("{}", ip);

    let location = if old_ip.is_none() || old_ip.unwrap() != ip_str {
        let mmdb_reader = request.guard::<State<maxminddb::Reader<Vec<u8>>>>()?;

        let location: String = match mmdb_reader.lookup::<geoip2::City>(ip) {
            Ok(lookup) => {
                let mut result = String::from("lookup failed");
                if let Some(entry) = lookup.city {
                    if let Some(names) = entry.names {
                        if let Some(name) = names.get("en") {
                            result = name.to_string();
                        }
                    }
                }
                result
            },
            Err(_) => String::from("unknown"),
        };
        Some(location)
    } else {
        None
    };
    Outcome::Success((user_agent, ip_str, location))
}

pub struct LoginGuard
{
    pub token: String,
    pub passed_with_pin: bool,
}

impl<'a, 'r> FromRequest<'a, 'r> for LoginGuard
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<LoginGuard, ()>
    {
        let headers = request.headers();

        let header: Vec<_> = headers.get("authorization").collect();

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

        let result = select_user_by_name_optional(credentials[0], &connection)
            .expect("selecting user by name");

        let user: User = match result {
            None => return Outcome::Failure((Status::Unauthorized, ())),
            Some(user) => user,
        };

        if user.locked {
            return Outcome::Failure((Status::Unauthorized, ()))
        }

        if PasswordHash::with_salt(credentials[1], &user.salt[..]) != user.hash {
            return Outcome::Failure((Status::Unauthorized, ()))
        }
        // check password for being pin
        let passed_with_pin = match select_user_pin_by_id(user.id, &connection) {
            Ok(Some(pin)) => format!("{}", pin) == credentials[1],
            _ => false,
        };

        // generate token
        let token = random_bytes(128);

        // using the password hash as salt for performace reasons
        // and so every token gets invalidated on password change
        let hash = GenericHash::with_salt(&token[..], &user.hash[..]);

        let x_api_key = match merge_x_api_key(user.id, token) {
            Ok(key) => key,
            Err(_) => return Outcome::Failure((Status::Unauthorized, ())),
        };

        // encode for client
        let x_api_key = base64::encode_config(&x_api_key[..], base64::URL_SAFE);

        let (user_agent, ip, location) = parse_header(request, None)?;

        // sanitize too large user agents
        match insert_into_user_tokens(
            user.id,
            &user_agent,
            &ip,
            &location.unwrap(), // we have a location here as we passed None to parse_header
            hash,
            &connection,
        ) {
            Ok(_) => {
                info!("{} logged in ", user.id);
                Outcome::Success(LoginGuard {
                    token: x_api_key,
                    passed_with_pin,
                })
            },
            Err(_) => {
                error!("failed to insert token into database");
                Outcome::Failure((Status::InternalServerError, ()))
            },
        }
    }
}
