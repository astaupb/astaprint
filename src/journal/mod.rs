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

pub mod response;

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
