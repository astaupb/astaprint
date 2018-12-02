/// AStAPrint - Journal
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
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{
    insert_into,
    prelude::*,
    r2d2::{
        ConnectionManager,
        PooledConnection,
    },
    QueryResult,
};
use journal::lock::JournalLock;
use r2d2_redis::{
    r2d2::Pool,
    RedisConnectionManager,
};

use astacrypto::generichash::GenericHash;
use journal::digest::{
    table::*,
    JournalDigest,
};
use std::{
    fmt,
    str::FromStr,
};

pub mod get;
pub mod post;

pub mod response;

pub mod credit;
pub mod digest;
pub mod lock;
pub mod tokens;

pub mod table;
use self::table::*;
#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal"]
pub struct Journal
{
    pub id: u32,
    pub user_id: u32,
    pub value: BigDecimal,
    pub description: String,
    pub created: NaiveDateTime,
}

impl fmt::Display for Journal
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}{}{}{}{}", self.id, self.user_id, self.value, self.description, self.created)
    }
}

pub fn insert(
    user_id: u32,
    value: BigDecimal,
    description: &str,
    redis: Pool<RedisConnectionManager>,
    mysql: PooledConnection<ConnectionManager<MysqlConnection>>,
) -> QueryResult<BigDecimal>
{
    let _lock = JournalLock::from(redis);

    insert_into(journal::table)
        .values((
            journal::user_id.eq(user_id),
            journal::value.eq(value),
            journal::description.eq(description),
        ))
        .execute(&mysql)?;

    let (digest, credit) = calculate_digest(user_id, &mysql)?;

    insert_into(journal_digest::table)
        .values((journal_digest::digest.eq(digest), journal_digest::credit.eq(credit.clone())))
        .execute(&mysql)?;

    Ok(credit)
}
pub fn calculate_digest(
    user_id: u32,
    mysql: &PooledConnection<ConnectionManager<MysqlConnection>>,
) -> QueryResult<(Vec<u8>, BigDecimal)>
{
    let last_entry: Option<u32> = journal::table
        .select(journal::id)
        .filter(journal::user_id.eq(user_id))
        .order(journal::id.desc())
        .offset(1)
        .first(mysql)
        .optional()?;

    let credit = match last_entry {
        None => BigDecimal::from_str("0.0").unwrap(),
        Some(id) => {
            journal_digest::table
                .select(journal_digest::credit)
                .filter(journal_digest::id.eq(id + 1))
                .first(mysql)?
        },
    };

    let journal: Journal =
        journal::table.select(journal::all_columns).order(journal::id.desc()).first(mysql)?;

    let seed: JournalDigest = journal_digest::table
        .select(journal_digest::all_columns)
        .order(journal_digest::id.desc())
        .first(mysql)?;

    assert_eq!(journal.id, seed.id);

    let new_digest = GenericHash::with_salt(&mut format!("{}", journal).as_bytes(), &seed.digest[..]);

    let new_credit = credit + journal.value;

    Ok((new_digest, new_credit))
}
#[cfg(test)]
mod tests
{
    use diesel::prelude::*;
    use journal::*;
    use pool::create_mysql_pool;
    use std::env;
    #[test]
    fn verify()
    {
        let url = env::var("ASTAPRINT_DATABASE_URL").unwrap();
        let connection = create_mysql_pool(&url, 1).get().unwrap();
        let journal: Vec<Journal> =
            journal::table.select(journal::all_columns).load(&connection).expect("selecting journal");

        let digests: Vec<JournalDigest> = journal_digest::table
            .select(journal_digest::all_columns)
            .load(&connection)
            .expect("selecting journal digests");

        for (i, entry) in journal.iter().enumerate() {
            let mut salt = digests[i].digest.clone();

            let input = format!("{}", entry);

            let hash = GenericHash::with_salt(input.as_bytes(), &salt[..]);

            if i < digests.len() + 1 {
                assert_eq!(&hash[..], &digests[i + 1].digest[..]);
            }
            println!("{} verified", i);
        }
    }
}
