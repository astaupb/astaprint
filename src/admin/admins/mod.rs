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

pub mod http;

use chrono::{
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
};
use diesel::prelude::*;
use mysql::admin::{
    insert::insert_admin,
    Admin,
};

#[derive(Deserialize, Debug, Clone)]
pub struct NewAdmin
{
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct AdminCreate
{
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub service: bool,
    pub locked: bool,
    pub expires: NaiveDate,
    pub created_by: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AdminUpdate
{
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub login: Option<String>,
    pub service: Option<bool>,
    pub locked: Option<bool>,
    pub expires: Option<i64>,
}

impl AdminUpdate
{
    pub fn update(self, mut admin: Admin) -> Admin
    {
        if let Some(first_name) = self.first_name {
            admin.first_name = first_name;
        }
        if let Some(last_name) = self.last_name {
            admin.last_name = last_name;
        }
        if let Some(login) = self.login {
            admin.login = login;
        }
        if let Some(service) = self.service {
            admin.service = service;
        }
        if let Some(locked) = self.locked {
            admin.locked = locked;
        }
        if let Some(expires) = self.expires {
            admin.expires = NaiveDateTime::from_timestamp(expires, 0 /* ns */).date();
        }
        admin
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AdminResponse
{
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub login: String,
    pub service: bool,
    pub locked: bool,
    pub expires: i64,
    pub created_by: Option<u32>,
    pub created: i64,
    pub updated: i64,
}

impl From<&Admin> for AdminResponse
{
    fn from(admin: &Admin) -> AdminResponse
    {
        AdminResponse {
            id: admin.id,
            first_name: admin.first_name.clone(),
            last_name: admin.last_name.clone(),
            login: admin.login.clone(),
            service: admin.service,
            locked: admin.locked,
            expires: admin.expires.and_time(NaiveTime::from_hms(0, 0, 0)).timestamp(),
            created_by: admin.created_by,
            created: admin.created.timestamp(),
            updated: admin.updated.timestamp(),
        }
    }
}

impl AdminCreate
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
