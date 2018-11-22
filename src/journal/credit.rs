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
use bigdecimal::{
    BigDecimal,
    ToPrimitive,
};

use diesel::{
    prelude::*,
    result::QueryResult,
};

use rocket_contrib::Json;

use journal::*;

use user::{
    *,
    guard::UserGuard,
};

#[get("/credit")]
pub fn credit(user: UserGuard) -> QueryResult<Json<f64>>
{
    let credit: BigDecimal = get_credit(user.id, &user.connection)?;

    info!("{} fetched credit {}", user.id, credit);

    Ok(Json(credit.to_f64().unwrap()))
}

pub fn get_credit(user_id: u32, connection: &MysqlConnection) -> QueryResult<BigDecimal>
{
    let mut credit_id: u32 = user::table
        .inner_join(journal::table)
        .select(journal::id)
        .filter(user::id.eq(journal::user_id))
        .filter(user::id.eq(user_id))
        .order(journal::id.desc())
        .first(connection)?;

    // calculated credit has offset of one from journal
    credit_id += 1;

    let credit: BigDecimal = journal_digest::table
        .select(journal_digest::credit)
        .filter(journal_digest::id.eq(credit_id))
        .first(connection)?;

    Ok(credit)
}
