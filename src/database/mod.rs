pub mod printer;
/// AStAPrint-Database
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
pub mod user;

pub use self::{printer::*,
               user::*};

use std::{env,
          ops::Deref};

use diesel::{prelude::*,
             r2d2::{ConnectionManager,
                    Pool,
                    PooledConnection}};

pub fn establish_connection() -> MysqlConnection
{
    let url = env::var("ASTAPRINT_DATABASE_URL").expect("reading ASTAPRINT_DATABASE_URL from environment");

    MysqlConnection::establish(&url).expect("establishing MysqlConnection")
}

type ConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn init_connection_pool() -> ConnectionPool
{
    let url = env::var("ASTAPRINT_DATABASE_URL").expect("reading ASTAPRINT_DATABASE_URL from environment");

    let manager = ConnectionManager::<MysqlConnection>::new(url);

    Pool::new(manager).expect("initialising MySqlPool")
}

pub struct PoolConnection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl Deref for PoolConnection
{
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

#[cfg(test)]

mod database_tests
{

    use super::printer::{select_device_ids,
                         select_printer_interface_information};

    #[test]

    fn dump_printer_interface_information()
    {
        for id in select_device_ids().iter() {
            println!("{}: {:?}", id, select_printer_interface_information(id));
        }
    }

    use super::{customer::{representation::Journal,
                           schema::{journal,
                                    journal_digest}},
                establish_connection};

    use chrono::Duration;
    use diesel::prelude::*;
    use sha2::{Digest,
               Sha512};

    #[test]

    fn verify_digests()
    {
        let connection = establish_connection();

        let mut digests: Vec<Vec<u8>> =
            journal_digest::table.select(journal_digest::digest).load(&connection).unwrap();

        let journal_rows: Vec<Journal> =
            journal::table.select(journal::all_columns).load(&connection).unwrap();

        for (i, row) in journal_rows.iter().enumerate() {
            // FIXME
            let diff = Duration::hours(2);

            let concat = format!(
                "{}{}{}{}{}{}",
                row.id,
                row.customer_id,
                row.value,
                row.credit,
                row.description,
                row.time + diff
            );

            let mut concat_bytes = concat.as_bytes().to_owned();

            digests[i].append(&mut concat_bytes);

            let out = Sha512::digest(&digests[i]);

            assert_eq!(out[..], digests[i + 1][..]);

            println!("row {} verified", i);
        }
    }

}
