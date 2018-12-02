/// AStAPrint Jobs - options
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
pub mod get;
pub mod put;

pub mod pagerange;

use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobOptions
{
    pub duplex: u8,
    pub copies: u16,
    pub collate: bool,
    pub keep: bool,
    pub a3: bool,
    pub nup: u8,
    pub nuppageorder: u8,
    pub range: String,
}

impl JobOptions
{
    pub fn serialize(&self) -> Vec<u8>
    {
        bincode::serialize(&self).expect("serializing JobOptions")
    }
}

impl Default for JobOptions
{
    fn default() -> JobOptions
    {
        JobOptions {
            duplex: 0,
            copies: 1,
            collate: false,
            keep: false,
            a3: false,
            nup: 1,
            nuppageorder: 0,
            range: String::from(""),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value
{
    S(String),
    I(u16),
    B(bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobOptionsUpdate
{
    pub duplex: Option<u8>,
    pub copies: Option<u16>,
    pub collate: Option<bool>,
    pub keep: Option<bool>,
    pub a3: Option<bool>,
    pub nup: Option<u8>,
    pub nuppageorder: Option<u8>,
    pub range: Option<String>,
}

pub trait Update
{
    fn merge(&mut self, update: JobOptionsUpdate);
}

impl Update for JobOptions
{
    fn merge(&mut self, update: JobOptionsUpdate)
    {
        if let Some(duplex) = update.duplex {
            self.duplex = duplex;
        }

        if let Some(copies) = update.copies {
            self.copies = copies;
        }

        if let Some(collate) = update.collate {
            self.collate = collate;
        }

        if let Some(keep) = update.keep {
            self.keep = keep;
        }

        if let Some(a3) = update.a3 {
            self.a3 = a3;
        }

        if let Some(nup) = update.nup {
            self.nup = nup;
        }

        if let Some(nuppageorder) = update.nuppageorder {
            self.nuppageorder = nuppageorder;
        }

        if let Some(range) = update.range {
            self.range = range;
        }
    }
}
