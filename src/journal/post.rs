/// AStAPrint
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
use rocket::{
    http::Status,
    State,
};
use rocket_contrib::json::Json;
use bigdecimal::{BigDecimal, ToPrimitive};

use diesel::prelude::*;

use r2d2_redis::{
    RedisConnectionManager,
    r2d2::Pool,
};

use user::guard::UserGuard;
use admin::guard::AdminGuard;

use legacy::tds::insert_transaction;

use journal::lock::JournalLock;

use mysql::journal::{
    JournalToken,
    select::select_journal_token_by_content,
    update::update_journal_token,
};

#[derive(Deserialize, Debug, Clone)]
pub struct JournalPost
{
    user_id: u32,
    value: i32,
    description: String,
    without_money: bool,
}

#[post("/", data = "<token>")]
fn post_to_journal_with_token(
    user: UserGuard,
    token: Json<String>,
    redis: State<Pool<RedisConnectionManager>>,
) -> QueryResult<Status>
{
    let token: Option<JournalToken> =
        select_journal_token_by_content(token.into_inner(), &user.connection)
            .expect("selecting journal token");

    match token {
        None => Ok(Status::new(401, "Unauthorized")),
        Some(token) => {
            if token.used {
                return Ok(Status::new(472, "Token Already Consumed"));
            }

            update_journal_token(token.id, true, user.id, &user.connection)?;

            let _lock = JournalLock::from(redis.clone());

            insert_transaction(
                user.id,
                (token.value * BigDecimal::from(100)).to_i32().unwrap(),
                &format!("created with token {}", token.content),
                false,
                None,
            );


            Ok(Status::new(204, "Success - No Content"))
        },
    }
}

#[post("/journal", data = "<body>")]
pub fn post_to_journal_as_admin(body: Json<JournalPost>, admin: AdminGuard, redis: State<Pool<RedisConnectionManager>>) -> Status {
    let _lock = JournalLock::from(redis.clone());

    insert_transaction(
        body.user_id,
        body.value,
        &body.description,
        body.without_money,
        Some(admin.id),
    );

    Status::new(204, "Success - No Content")
}
