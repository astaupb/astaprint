/// AStAPrint - Admin
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
use chrono::{
    NaiveDate,
    NaiveDateTime,
};
use self::table::*;

#[derive(Identifiable, Queryable, Insertable, Debug)]
#[table_name = "admin"]
pub struct Admin
{
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub is_service: bool,
    pub expires: NaiveDate,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "admin_tokens"]
pub struct AdminToken
{
    pub id: u32,
    pub user_id: u32,
    pub user_agent: String,
    pub ip: String,
    pub location: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}
