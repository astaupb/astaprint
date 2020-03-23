// AStAPrint
// Copyright (C) 2018, 2019, 2020 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
// Daniel Negi
// <daniel.negi@asta.upb.de>
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
use chrono::NaiveDateTime;
use mysql::admin::Admin;

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
