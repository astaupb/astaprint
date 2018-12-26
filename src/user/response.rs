/// AStAPrint-Backend - User Responses
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

use mysql::user::UserToken;

#[derive(Serialize, Debug)]
pub struct UserTokenResponse
{
    pub id: u32,
    pub user_agent: String,
    pub ip: String,
    pub location: String,
    pub created: String,
}

impl<'a> From<&'a UserToken> for UserTokenResponse
{
    fn from(row: &UserToken) -> UserTokenResponse
    {
        UserTokenResponse {
            id: row.id,
            user_agent: row.user_agent.clone(),
            ip: row.ip.clone(),
            location: row.location.clone(),
            created: format!("{}", row.created),
        }
    }
}
