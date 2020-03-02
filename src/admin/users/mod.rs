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

use model::job::options::JobOptions;
use mysql::user::User;

#[derive(Serialize, Debug, Clone)]
pub struct UserResponse
{
    pub id: u32,
    pub name: String,
    pub credit: i32,
    pub options: Option<JobOptions>,
    pub card: Option<u64>,
    pub pin: Option<u32>,
    pub locked: bool,
    pub email: Option<String>,
    pub created: i64,
    pub updated: i64,
}

impl<'a> From<&'a User> for UserResponse
{
    fn from(user: &User) -> UserResponse
    {
        UserResponse {
            id: user.id,
            name: user.name.clone(),
            credit: user.credit,
            options: user
                .options
                .clone()
                .map(|x| bincode::deserialize(&x[..]).expect("deserializing JobOption")),
            card: user.card,
            pin: user.pin,
            locked: user.locked,
            email: user.email.clone(),
            created: user.created.timestamp(),
            updated: user.updated.timestamp(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Card
{
    sn: Option<u64>,
    pin: Option<u32>,
}
