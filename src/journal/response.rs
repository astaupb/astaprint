/// AStAPrint-Backend - Journal Response
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
use bigdecimal::ToPrimitive;

use crate::journal::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct JournalResponse
{
    pub value: f64,
    // TODO: zip with journal_digest for credit
    // pub credit: f64,
    pub description: String,
    pub created: String,
}

impl<'a> From<&'a Journal> for JournalResponse
{
    fn from(journal: &Journal) -> JournalResponse
    {
        JournalResponse {
            value: journal.value.to_f64().unwrap(),
            // credit: journal.credit.to_f64().unwrap(),
            description: journal.description.clone(),
            created: format!("{}", journal.created),
        }
    }
}
