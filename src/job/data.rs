/// AStAPrint Job - data.rs
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
use chrono::Local;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct ShortJobData
{
    pub uid: String,
    pub user_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct JobData
{
    pub uid: String,
    pub user_id: u32,
    pub timestamp: i64,
    pub info: JobInfo,
    pub options: JobOptions,
}

impl JobData
{
    pub fn new(uid: &str, user_id: u32, filename: &str, password: &str, color: bool) -> JobData
    {
        JobData {
            uid: String::from(uid),
            user_id,
            timestamp: Local::now().timestamp(),
            info: JobInfo::new(filename, password, color),
            options: JobOptions::default(),
        }
    }
}

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
    fn new(filename: &str, password: &str, color: bool) -> JobInfo
    {
        JobInfo {
            filename: String::from(filename),
            pagecount: 0,
            color,
            a3: false,
            password: String::from(password),
        }
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

impl JobData
{
    pub fn pages_to_print(&self) -> u16
    {
        let mut count = self.info.pagecount;

        count = (count / u16::from(self.options.nup)) + match self.info.pagecount
            % u16::from(self.options.nup)
        {
            0 => 0,
            _ => 1,
        };

        if self.options.a3 {
            count *= 2;
        }

        count * self.options.copies
    }
}

#[test]
fn pages_to_print()
{
    let mut data = JobData::new("uid", 1, "filename", "password", true);
    data.info.pagecount = 18;
    data.options.nup = 4;
    println!("{:?}, pages to print: {}", data, data.pages_to_print());
}
