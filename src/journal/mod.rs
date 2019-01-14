/// AStAPrint - Journal
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
use bigdecimal::BigDecimal;
use diesel::{
    prelude::*,
    QueryResult,
};
use journal::lock::JournalLock;
use r2d2_redis::{
    r2d2::Pool,
    RedisConnectionManager,
};

use sodium::generichash::GenericHash;

use mysql::journal::{
    insert::*,
    select::*,
    Journal,
    JournalDigest,
};
use std::str::FromStr;

pub mod get;
pub mod post;

pub mod response;

pub mod credit;
pub mod lock;

pub fn insert(
    user_id: u32,
    value: BigDecimal,
    description: &str,
    redis: Pool<RedisConnectionManager>,
    connection: &MysqlConnection,
) -> QueryResult<BigDecimal>
{
    let _lock = JournalLock::from(redis);

    insert_into_journal(user_id, value, description, connection)?;

    let (digest, credit) = calculate_digest(user_id, connection)?;

    insert_into_journal_digest(digest, credit.clone(), connection)?;

    Ok(credit)
}
pub fn calculate_digest(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<(Vec<u8>, BigDecimal)>
{
    let last_entry: Option<u32> =
        select_latest_journal_id_of_user(user_id, connection)?;

    let credit = match last_entry {
        None => BigDecimal::from_str("0.0").unwrap(),
        Some(id) => select_credit_by_id(id, connection)?,
    };

    let journal: Journal = select_latest_journal_entry(connection)?;

    let seed: JournalDigest = select_latest_journal_digest(connection)?;
    assert_eq!(journal.id, seed.id);

    let new_digest = GenericHash::with_salt(
        &mut format!("{}", journal).as_bytes(),
        &seed.digest[..],
    );

    let new_credit = credit + journal.value;

    Ok((new_digest, new_credit))
}
#[cfg(test)]
mod tests
{
    use mysql::{
        create_mysql_pool,
        journal::{
            select::*,
            *,
        },
    };
    use sodium::GenericHash;
    use std::env;
    #[test]
    fn verify()
    {
        let url = env::var("ASTAPRINT_DATABASE_URL")
            .expect("getting database url from environment");
        let connection = create_mysql_pool(&url, 1).get().unwrap();
        let journal: Vec<Journal> =
            select_journal(&connection).expect("selecting journal");

        let digests: Vec<JournalDigest> = select_journal_digest(&connection)
            .expect("selecting journal digest");

        for (i, entry) in journal.iter().enumerate() {
            let mut salt = digests[i].digest.clone();

            let input = format!("{}", entry);

            let hash = GenericHash::with_salt(input.as_bytes(), &salt[..]);

            assert_eq!(&hash[..], &digests[i + 1].digest[..]);
            println!("id {} verified", i + 1);
        }
    }
}
