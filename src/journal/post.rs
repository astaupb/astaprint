/// AStAPrint
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
use diesel::{
    prelude::*,
    result::QueryResult,
    update,
};
use r2d2_redis::{
    r2d2::Pool,
    RedisConnectionManager,
};

use rocket::{
    http::Status,
    response::{
        status::NoContent,
        Responder,
        Response,
    },
    Request,
    State,
};
use rocket_contrib::Json;

use user::guard::UserGuard;

use journal::{
    insert,
};

use mysql::journal::{
    Journal, select::*,
};

#[derive(Debug)]
pub enum ErrorKind
{
    TokenAlreadyConsumed,
    Unauthorized,
}
use self::ErrorKind::*;

#[derive(Debug)]
pub struct JournalPostError(ErrorKind);
impl<'r> Responder<'r> for JournalPostError
{
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status>
    {
        Response::build()
            .status(match self.0 {
                TokenAlreadyConsumed => Status::new(472, "token already consumend"),
                Unauthorized => Status::Unauthorized,
            })
            .ok()
    }
}

#[post("/", data = "<token>")]
fn post_to_journal(
    user: UserGuard,
    token: Json<String>,
    redis: State<Pool<RedisConnectionManager>>,
) -> QueryResult<Result<NoContent, JournalPostError>>
{
    let token: Option<JournalToken> = select_journal_token_by_value(token.into_inner(), &user.connection)
        .expect("selecting journal token");

    match token {
        None => return Ok(Err(JournalPostError(Unauthorized))),
        Some(token) => {
            if token.used {
                return Ok(Err(JournalPostError(TokenAlreadyConsumed)));
            }
            update(journal_tokens::table.filter(journal_tokens::id.eq(token.id)))
                .set((journal_tokens::used.eq(true), journal_tokens::used_by.eq(user.id)))
                .execute(&user.connection)?;

            insert(
                user.id,
                token.value,
                &format!("created with token {}", token.content),
                redis.inner().clone(),
                user.connection,
            )?;


            Ok(Ok(NoContent))
        },
    }
}
