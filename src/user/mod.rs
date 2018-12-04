/// AStAPrint - User
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
use chrono::NaiveDateTime;

pub mod table;
use self::table::*;

pub mod get;
pub mod post;
pub mod put;

pub mod add;
pub mod guard;
pub mod key;
pub mod login;
pub mod response;
pub mod tokens;

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "user"]
pub struct User
{
    pub id: u32,
    pub name: String,
    pub locked: bool,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub card: Option<u64>,
    pub pin: Option<u32>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}
