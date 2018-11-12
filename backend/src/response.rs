/// AStAPrint-Backend - Database
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
use astaprint::database::user::representation::*;
use bigdecimal::ToPrimitive;

use base64;

#[derive(Serialize, Deserialize, Debug)]

pub struct JournalResponse
{
    pub value: f64,
    pub credit: f64,
    pub description: String,
    pub timestamp: i64,
}

impl<'a> From<&'a Journal> for JournalResponse
{
    fn from(journal: &Journal) -> JournalResponse
    {
        JournalResponse {
            value: journal.value.to_f64().unwrap(),
            credit: journal.credit.to_f64().unwrap(),
            description: journal.description.clone(),
            timestamp: journal.timestamp.timestamp(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TokenResponse
{
    pub token: String,
    pub user_agent: String,
    pub location: String,
    pub timestamp: String,
}

impl<'a> From<&'a Token> for TokenResponse
{
    fn from(row: &Token) -> TokenResponse
    {
        TokenResponse {
            token: String::from(&base64::encode_config(&row.value[..], base64::URL_SAFE)[..8]),
            user_agent: row.user_agent.clone(),
            location: row.location.clone(),
            timestamp: format!("{}", row.timestamp),
        }
    }
}
