/// AStAPrint - Jobs
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
use chrono::{
    NaiveDateTime,
    Local,
};

use crate::jobs::{
    info::JobInfo,
    options::JobOptions,
};

use mysql::jobs::{
    Job as JobRow,
    select::*,
};

pub mod info;
pub mod options;
pub mod data;

pub mod pdf;
pub mod task;
pub mod tmp;
pub mod queue;

pub mod select;
pub mod get;
pub mod delete;
pub mod response;


#[derive(Serialize, Deserialize)]
pub struct Job
{
    pub id: u32,
    pub user_id: u32,
    pub timestamp: i64,
    pub info: JobInfo,
    pub options: JobOptions,
}

impl From<(u32, Vec<u8>, Vec<u8>, NaiveDateTime)> for Job
{
    fn from(row: (u32, Vec<u8>, Vec<u8>, NaiveDateTime)) -> Job
    {
        Job {
            id: row.0,
            info: bincode::deserialize(&row.1[..]).expect("deserializing JobInfo"),
            options: bincode::deserialize(&row.2[..]).expect("deserializing JobOptions"),
            timestamp: row.3.timestamp(),
        }
    }
}

impl Job
{
    pub fn translate_for_printer(&mut self, uid: &[u8], data: Vec<u8>) -> Vec<u8>
    {
        let info = self.info();
        let options = self.options();

        let mut header: Vec<u8> = Vec::with_capacity(8096);
        header.append(&mut
                  b"\x1b\x25\x2d\x31\x32\x33\x34\x35\
                    \x58\x40\x50\x4a\x4c\x20\x43\x4f\
                    \x4d\x4d\x45\x4e\x54\x20\x50\x4a\
                    \x4c\x2c\x4c\x49\x4e\x55\x58\x2c\
                    \x50\x44\x46\r\n".to_vec(),
        );

        header.append(&mut format!(
                   "\x40\x50\x4a\x4c\x20\x4a\x4f\x42\
                    \x20\x4e\x41\x4d\x45\x3d\"{}\"\r\
                    \n", self.user_id
            ).as_bytes().to_owned(),
        );

        header.append(&mut format!(
                   "\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x4a\x4f\x42\x4e\x41\x4d\x45\
                    \x3d\"{}\"\r\n", self.user_id
            ).as_bytes().to_owned(),
        );

        header.append(&mut format!(
                   "\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x54\x52\x41\x43\x4b\x49\x44\
                    \x3d\"{}\"\r\n", hex::encode(uid)
            ).as_bytes().to_owned(),
        );

        header.append(&mut format!(
                   "\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x55\x53\x45\x52\x4e\x41\x4d\
                    \x45\x3d\"{}\"\r\n", self.user_id
            ).as_bytes().to_owned(),
        );

        let dt = Local::now().format("%Y/%m/%d%H:%M:%S").to_string();

        let (date, time) = dt.split_at(10);

        header.append(&mut format!(
                   "\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x44\x41\x54\x45\x3d\"{}\"\
                    \r\n\x40\x50\x4a\x4c\x20\x53\x45\
                    \x54\x20\x54\x49\x4d\x45\x3d\"{}\
                    \"\r\n", date, time
            ).as_bytes().to_owned(),
        );

        if !options.a3 {
            header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x50\x41\x50\x45\x52\x3d\x41\
                    \x34\r\n".to_vec(),
            );
        } else {
            header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x50\x41\x50\x45\x52\x3d\x41\
                    \x33\r\n".to_vec(),
            );
        }

        match options.duplex {
            0 => {
                header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x44\x55\x50\x4c\x45\x58\x3d\
                    \x4f\x46\x46\r\n".to_vec(),
                );
            },
            1 => {
                header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x44\x55\x50\x4c\x45\x58\x3d\
                    \x4f\x4e\r\n\x40\x50\x4a\x4c\x20\
                    \x53\x45\x54\x20\x42\x49\x4e\x44\
                    \x49\x4e\x47\x3d\x4c\x4f\x4e\x47\
                    \x45\x44\x47\x45\r\n".to_vec(),
                );
            },
            2 => {
                header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x44\x55\x50\x4c\x45\x58\x3d\
                    \x4f\x4e\r\n\x40\x50\x4a\x4c\x20\
                    \x53\x45\x54\x20\x42\x49\x4e\x44\
                    \x49\x4e\x47\x3d\x53\x48\x4f\x52\
                    \x54\x45\x44\x47\x45\r\n".to_vec(),
                );
            },
            _ => (),
        }

        if options.copies > 1 {
            if options.collate {
                // WHAT THE FUCK RICOH
                header.append(&mut format!(
                   "\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x43\x4f\x50\x49\x45\x53\x3d\
                    {}\r\n",
                    options.copies
                ).as_bytes().to_owned(),);
            } else {
                header.append(&mut format!(
                   "\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x51\x54\x59\x3d{}\r\n",
                    options.copies
                ).as_bytes().to_owned(),);
            }
        }

        if options.a3 != info.a3 {
            if !options.a3 {
                header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x46\x49\x54\x54\x4f\x50\x41\
                    \x47\x45\x53\x49\x5a\x45\x3d\x41\
                    \x34\r\n".to_vec(),);
            } else {
                header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x46\x49\x54\x54\x4f\x50\x41\
                    \x47\x45\x53\x49\x5a\x45\x3d\x41\
                    \x33\r\n".to_vec(),);
            }
        }

        if options.nup > 1 {
            match options.nup {
                2 => {
                    header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x4e\x55\x50\x3d\x32\r\n".to_vec(),);
                },
                4 => {
                    header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x4e\x55\x50\x3d\x34\r\n".to_vec(),);
                },
                _ => (),
            }
            match options.nuppageorder {
                0 => {
                    header.append(&mut
                  b"\x50\x4a\x4c\x20\x53\x45\x54\x20\
                    \x4e\x55\x50\x50\x41\x47\x45\x4f\
                    \x52\x44\x45\x52\x3d\x52\x49\x47\
                    \x48\x54\x54\x48\x45\x4e\x44\x4f\
                    \x57\x4e\r\n".to_vec(),);
                },
                1 => {
                    header.append(&mut
                  b"\x50\x4a\x4c\x20\x53\x45\x54\x20\
                    \x4e\x55\x50\x50\x41\x47\x45\x4f\
                    \x52\x44\x45\x52\x3d\x44\x4f\x57\
                    \x4e\x54\x48\x45\x4e\x52\x49\x47\
                    \x48\x54\r\n".to_vec(),);
                },
                2 => {
                    header.append(&mut
                  b"\x50\x4a\x4c\x20\x53\x45\x54\x20\
                    \x4e\x55\x50\x50\x41\x47\x45\x4f\
                    \x52\x44\x45\x52\x3d\x4c\x45\x46\
                    \x54\x54\x48\x45\x4e\x44\x4f\x57\
                    \x4e\r\n".to_vec(),);
                },
                3 => {
                    header.append(&mut
                  b"\x50\x4a\x4c\x20\x53\x45\x54\x20\
                    \x4e\x55\x50\x50\x41\x47\x45\x4f\
                    \x52\x44\x45\x52\x3d\x44\x4f\x57\
                    \x4e\x54\x48\x45\x4e\x4c\x45\x46\
                    \x54\r\n".to_vec(),);
                },
                _ => (),
            }
        }

        if options.range != "" {
            // assuming pagerange is valid here
            header.append(&mut format!(
                   "\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x50\x52\x49\x4e\x54\x50\x41\
                    \x47\x45\x53\x3d\"{}\"\r\n",
                    options.range
                ).as_bytes().to_owned(),
            );
        }

        // set defaults for sanity
        header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x41\x55\x54\x4f\x54\x52\x41\
                    \x59\x43\x48\x41\x4e\x47\x45\x20\
                    \x4f\x4e\r\n".to_vec(),
        );

        header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x53\x45\x54\
                    \x20\x4d\x45\x44\x49\x41\x54\x59\
                    \x50\x45\x3d\x50\x4c\x41\x49\x4e\
                    \x4e\x4f\x52\x45\x43\x59\x43\x4c\
                    \x45\x44\r\n".to_vec(),
        );
        header.append(&mut
                  b"\x40\x50\x4a\x4c\x20\x45\x4e\x54\
                    \x45\x52\x20\x4c\x41\x4e\x47\x55\
                    \x41\x47\x45\x3d\x50\x44\x46\r\n"
                    .to_vec(),
        );

        header.append(&mut data);

        header.append(&mut
                  b"\x1b\x25\x2d\x31\x32\x33\x34\x35\
                    \x58\x40\x50\x4a\x4c\x20\x45\x4f\
                    \x4a\r\n\x1b\x25\x2d\x31\x32\x33\
                    \x34\x35\x58".to_vec(),
        );
        header
    }

}
