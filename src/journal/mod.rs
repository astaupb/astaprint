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

pub mod credit;
pub mod response;
pub mod get;

table! {
    journal (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        value -> Decimal,
        description -> Varchar,
        created -> Timestamp,
    }
}

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

table! {
    journal_digest (id) {
        id -> Unsigned<Integer>,
        digest -> Binary,
        credit -> Decimal,
        created -> Timestamp,
    }
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal_digest"]
pub struct JournalDigest
{
    pub id: u32,
    pub digest: Vec<u8>,
    pub credit: BigDecimal,
    pub created: NaiveDateTime,
}

#[cfg(test)]
mod tests
{
    use diesel::prelude::*;
    use pool::create_mysql_pool;
    use journal::*;
    extern crate sha2;
    use chrono::FixedOffset;
    use sha2::{
        Digest,
        Sha512,
    };
    use std::env;
    #[test]
    fn verify()
    {
        let url = env::var("ASTAPRINT_DATABASE_URL").unwrap();
        let connection = create_mysql_pool(&url, 1).get().unwrap();
        let journal: Vec<Journal> =
            journal::table.select(journal::all_columns).load(&connection).expect("selecting journal");
        println!("{:?}", journal);

        let digests: Vec<JournalDigest> = journal_digest::table
            .select(journal_digest::all_columns)
            .load(&connection)
            .expect("selecting journal digests");

        for (i, entry) in journal.iter().enumerate() {
            let mut input = digests[i].digest.clone();

            let datetime = entry.created + FixedOffset::west(-3600);

            let concat = format!(
                "{}{}{}{}{}",
                &entry.id, &entry.user_id, &entry.value, &entry.description, datetime,
            );

            println!("{}", concat);

            input.append(&mut concat.as_bytes().to_owned());
            println!("{:x?}", input);

            let mut hasher = Sha512::new();
            hasher.input(&input[..]);
            let result = hasher.result();

            println!("{:x?} == {:x?}", result.as_slice(), &digests[i+1].digest[..]);
            assert_eq!(result.as_slice(), &digests[i+1].digest[..]);
        }
    }
}
