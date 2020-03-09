// AStAPrint
// Copyright (C) 2018, 2019, 2020 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//          Daniel Negi <daniel.negi@asta.upb.de>
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
use mysql::admin::AdminToken;

#[derive(Serialize, Debug)]
pub struct AdminTokenResponse
{
    pub id: u32,
    pub user_agent: String,
    pub ip: String,
    pub location: String,
    pub created: i64,
    pub updated: i64,
}

impl<'a> From<&'a AdminToken> for AdminTokenResponse
{
    fn from(row: &AdminToken) -> AdminTokenResponse
    {
        AdminTokenResponse {
            id: row.id,
            user_agent: row.user_agent.clone(),
            ip: row.ip.clone(),
            location: row.location.clone(),
            created: row.created.timestamp(),
            updated: row.updated.timestamp(),
        }
    }
}
