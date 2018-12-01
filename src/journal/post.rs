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
    insert_into,
    prelude::*,
    result::QueryResult,
    update,
};

use rocket::{
    http::Status,
    response::{
        status::NoContent,
        Responder,
        Response,
    },
    Request,
};
use rocket_contrib::Json;

use user::guard::UserGuard;

use journal::{
    table::*,
    tokens::{
        table::*,
        JournalToken,
    },
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
fn post_to_journal(user: UserGuard, token: Json<String>)
    -> QueryResult<Result<NoContent, JournalPostError>>
{
    let token: Option<JournalToken> = journal_tokens::table
        .select(journal_tokens::all_columns)
        .filter(journal_tokens::content.eq(token.into_inner()))
        .first(&user.connection)
        .optional()?;

    match token {
        None => return Ok(Err(JournalPostError(Unauthorized))),
        Some(token) => {
            if token.used {
                return Ok(Err(JournalPostError(TokenAlreadyConsumed)));
            }
            update(journal_tokens::table)
                .set((journal_tokens::used.eq(true), journal_tokens::used_by.eq(user.id)))
                .execute(&user.connection)?;

            insert_into(journal::table)
                .values((
                    journal::value.eq(token.value),
                    journal::user_id.eq(user.id),
                    journal::description.eq(format!("created with token: {}", token.content)),
                ))
                .execute(&user.connection)?;

            Ok(Ok(NoContent))
        },
    }
}
