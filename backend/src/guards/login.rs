/// AStAPrint-Backend - Request Guards - Login
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
    insert_into,
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use astaprint::database::user::schema::*;

use crypto::{
    urandom,
    verify_password,
};

pub struct Login
{
    pub token: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Login
{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Login, ()>
    {
        let header = request.headers();

        let remote = request.remote().expect("reading remote address");

        let user_agent: Vec<_> = header.get("user-agent").collect();

        let header: Vec<_> = header.get("authorization").collect();

        let auth: Vec<&str> = header[0].split(' ').collect();

        if auth.len() != 2 && auth[0] != "Basic" {
            return Outcome::Failure((Status::BadRequest, ()));
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
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let pool = request.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;

        let connection = pool.get().expect("retrieving connection from pool");

        let mut result: Vec<(bool, u32, Vec<u8>, Vec<u8>)> = user::table
            .select((user::locked, user::id, user::password_hash, user::password_salt))
            .filter(user::name.eq(credentials[0]))
            .load(&connection)
            .expect("loading user status from table");

        if result.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let (locked, user_id, password_hash, password_salt) = result.pop().unwrap();

        if locked || !verify_password(credentials[1], &password_hash, &password_salt) {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        // generate token
        let mut buf: [u8; 64] = [0; 64];
        urandom(&mut buf);

        let token = base64::encode_config(&buf[..], base64::URL_SAFE);

        let user_agent = if user_agent[0].len() > 128 {
            String::from(&user_agent[0][..128])
        } else {
            String::from(user_agent[0])
        };
        match insert_into(token::table)
            .values((
                token::user_id.eq(user_id),
                token::user_agent.eq(user_agent),
                // FIXME: read city from geoip database
                token::location.eq(format!("{}", remote)),
                token::value.eq(Vec::from(&buf[..])),
            ))
            .execute(&connection)
        {
            Ok(_) => {
                info!("{} logged in ", user_id);
                Outcome::Success(Login {
                    token,
                })
            },
            Err(_) => {
                error!("failed to insert token into database");
                Outcome::Failure((Status::InternalServerError, ()))
            },
        }
    }
}
