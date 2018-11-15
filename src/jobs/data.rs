/// AStAPrint Jobs - options.rs
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
use bincode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobInfo
{
    pub filename: String,
    pub pagecount: u16,
    pub color: bool,
    pub a3: bool,
    pub password: String,
}

impl JobInfo
{
    pub fn new(filename: &str, password: &str, color: bool) -> JobInfo
    {
        JobInfo {
            filename: String::from(filename),
            pagecount: 0,
            color,
            a3: false,
            password: String::from(password),
        }
    }

    pub fn serialize(&self) -> Vec<u8>
    {
        bincode::serialize(&self).expect("serializing JobInfo")
    }
}

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
