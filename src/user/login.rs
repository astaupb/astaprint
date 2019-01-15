/// AStAPrint-Backend - Request Guards - Login
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
/// License
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


pub struct LoginGuard
{
    pub token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for LoginGuard
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<LoginGuard, ()>
    {
        let header = request.headers();

        let remote = request.remote().expect("reading remote address");

        let user_agent: Vec<_> = header.get("user-agent").collect();

        if user_agent.is_empty() {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        let header: Vec<_> = header.get("authorization").collect();

        if header.is_empty() {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let auth: Vec<&str> = header[0].split(' ').collect();

        if auth.len() != 2 && auth[0] != "Basic" {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let decoded: Vec<u8> =
            match base64::decode_config(auth[1], base64::URL_SAFE) {
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
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let pool =
            request.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;

        let connection = pool.get().expect("retrieving connection from pool");

        let result = select_user_by_name_optional(credentials[0], &connection)
            .expect("selecting user by name");

        let user: User = match result {
            None => return Outcome::Failure((Status::Unauthorized, ())),
            Some(user) => user,
        };

        if user.locked {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        if PasswordHash::with_salt(credentials[1], &user.salt[..]) != user.hash {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

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

        // sanitize too large user agents
        let user_agent = if user_agent[0].len() > 128 {
            String::from(&user_agent[0][..128])
        } else {
            String::from(user_agent[0])
        };

        let mmdb_reader =
            request.guard::<State<maxminddb::OwnedReaderFile<'_>>>()?;

        let city: String = match mmdb_reader.lookup::<geoip2::City>(remote.ip()) {
            Ok(city) => {
                city
                    .city
                    .expect("getting city entry from city record")
                    .names
                    .expect("getting names from city entry")
                    .get("en")
                    .expect("getting english entry from names_map")
                    .to_string()
            },
            Err(_) => String::from("unknown"),
        };


        match insert_into_user_tokens(
            user.id,
            &user_agent,
            &format!("{}", remote.ip()),
            &city,
            hash,
            &connection,
        ) {
            Ok(_) => {
                info!("{} logged in ", user.id);
                Outcome::Success(LoginGuard {
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
