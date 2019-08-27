// AStAPrint
// Copyright (C) 2018, 2019 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//
// This file is part of AStAPrint
//
// AStAPrint is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
pub mod guard;
pub mod login;
pub mod tokens;

pub mod get;
pub mod post;
pub mod put;

use chrono::NaiveDate;
use diesel::prelude::*;
use mysql::admin::insert::insert_admin;

#[derive(Debug, Clone)]
pub struct Admin
{
    pub first_name: String,
    pub last_name: String,
    pub login: Option<String>,
    pub hash: Option<Vec<u8>>,
    pub salt: Option<Vec<u8>>,
    pub service: bool,
    pub locked: bool,
    pub expires: NaiveDate,
    pub created_by: Option<u32>,
}

impl Admin
{
    pub fn insert(self, connection: &MysqlConnection) -> QueryResult<usize>
    {
        insert_admin(
            (
                self.first_name,
                self.last_name,
                self.login,
                self.hash,
                self.salt,
                self.service,
                self.locked,
                self.expires,
                self.created_by,
            ),
            connection,
        )
    }
}
