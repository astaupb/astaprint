/// AStAPrint-Database - User Representation
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
use super::schema::*;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal"]

pub struct Journal
{
    pub id: u32,
    pub user_id: u32,
    pub value: BigDecimal,
    pub credit: BigDecimal,
    pub description: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal_digest"]

pub struct JournalDigest
{
    pub id: u32,
    pub digest: Vec<u8>,
    pub timestamp: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "user"]

pub struct User
{
    pub id: u32,
    pub name: String,
    pub locked: bool,
    pub pin_hash: Option<Vec<u8>>,
    pub pin_salt: Option<Vec<u8>>,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub timestamp: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "token"]

pub struct Token
{
    pub id: u32,
    pub user_id: u32,
    pub user_agent: String,
    pub location: String,
    pub value: Vec<u8>,
    pub timestamp: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "register_token"]

pub struct RegisterToken
{
    pub id: u16,
    pub value: String,
    pub used: bool,
    pub user_id: Option<u32>,
    pub timestamp: NaiveDateTime,
}
