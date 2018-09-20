/// AStAPrint-Common - Data.rs
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

use pagerange::page_range_is_valid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShortJobData {
    pub uid: String,
    pub user_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobData {
    pub uid: String,
    pub user_id: u32,
    pub timestamp: i64,
    pub info: JobInfo,
    pub options: JobOptions,
}

impl JobData {
    pub fn new(uid: &str, user_id: u32, filename: &str, password: &str, color: bool) -> JobData {
        JobData{
            uid: String::from(uid),
            user_id: user_id,
            timestamp: Local::now().timestamp(),
            info: JobInfo::new(filename, password, color),
            options: JobOptions::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobInfo {
    pub filename: String,
    pub pagecount: u16,
    pub color: bool,
    pub a3: bool,
    pub password: String,
}

impl JobInfo {
    fn new(filename: &str, password: &str, color: bool) -> JobInfo {
        JobInfo{
            filename: String::from(filename),
            pagecount: 0,
            color: color,
            a3: false,
            password: String::from(password),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobOptions {
    pub duplex: u8,
    pub copies: u16,
    pub collate: bool,
    pub keep: bool,
    pub a3: bool,
    pub nup: u8,
    pub nuppageorder: u8,
    pub range: String,
}

impl Default for JobOptions {
    fn default() -> JobOptions {
        JobOptions{
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

impl JobData {
    pub fn get_pjl_header(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(256);

        buf.append(&mut b"\x1b\x25\x2d\x31\x32\x33\x34\x35\x58\x40\x50\x4a\x4c\x20\x43\x4f\x4d\x4d\x45\x4e\x54\x20\x50\x4a\x4c\x2c\x4c\x49\x4e\x55\x58\x2c\x50\x44\x46\r\n".to_vec());

        buf.append(
            &mut format!(
                "\x40\x50\x4a\x4c\x20\x4a\x4f\x42\x20\x4e\x41\x4d\x45\x3d\"{}\"\r\n",
                &self.uid
            ).as_bytes()
                .to_owned(),
        );

        buf.append(
            &mut format!(
                "\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x4a\x4f\x42\x4e\x41\x4d\x45\x3d\"{}\"\r\n",
                &self.uid
            ).as_bytes()
                .to_owned(),
        );

        buf.append(
            &mut format!(
                "\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x54\x52\x41\x43\x4b\x49\x44\x3d\"{}\"\r\n",
                &self.uid
            ).as_bytes()
                .to_owned(),
        );

        buf.append(
            &mut format!(
                "\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x55\x53\x45\x52\x4e\x41\x4d\x45\x3d\"{}\"\r\n",
                &self.user_id
            ).as_bytes()
                .to_owned(),
        );

        let dt = Local::now().format("%Y/%m/%d%H:%M:%S").to_string();
        let (date, time) = dt.split_at(10);
        buf.append(
            &mut format!(
                "\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x44\x41\x54\x45\x3d\"{}\"\r\n\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x54\x49\x4d\x45\x3d\"{}\"\r\n",
                date, time
            ).as_bytes()
                .to_owned(),
        );

        match self.options.a3 {
            false => {
                buf.append(&mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x50\x41\x50\x45\x52\x3d\x41\x34\r\n".to_vec());
            }
            true => {
                buf.append(&mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x50\x41\x50\x45\x52\x3d\x41\x33\r\n".to_vec());
            }
        }

        match self.options.duplex {
            0 => {
                buf.append(&mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x44\x55\x50\x4c\x45\x58\x3d\x4f\x46\x46\r\n".to_vec());
            }
            1 => {
                buf.append(&mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x44\x55\x50\x4c\x45\x58\x3d\x4f\x4e\r\n\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x42\x49\x4e\x44\x49\x4e\x47\x3d\x4c\x4f\x4e\x47\x45\x44\x47\x45\r\n".to_vec());
            }
            2 => {
                buf.append(&mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x44\x55\x50\x4c\x45\x58\x3d\x4f\x4e\r\n\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x42\x49\x4e\x44\x49\x4e\x47\x3d\x53\x48\x4f\x52\x54\x45\x44\x47\x45\r\n".to_vec());
            }
            _ => (),
        }

        if self.options.copies > 1 {
            match self.options.collate {
                // WHAT THE FUCK RICOH
                true => {
                    buf.append(
                        &mut format!(
                            "\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x43\x4f\x50\x49\x45\x53\x3d{}\r\n",
                            self.options.copies
                        ).as_bytes()
                            .to_owned(),
                    );
                }
                false => {
                    buf.append(
                        &mut format!(
                            "\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x51\x54\x59\x3d{}\r\n",
                            self.options.copies
                        ).as_bytes()
                            .to_owned(),
                    );
                }
            }
        }

        if self.options.a3 != self.info.a3 {
            match self.options.a3 {
                false => {
                    buf.append(
                        &mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x46\x49\x54\x54\x4f\x50\x41\x47\x45\x53\x49\x5a\x45\x3d\x41\x34\r\n".to_vec(),
                    );
                }
                true => {
                    buf.append(
                        &mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x46\x49\x54\x54\x4f\x50\x41\x47\x45\x53\x49\x5a\x45\x3d\x41\x33\r\n".to_vec(),
                    );
                }
            }
        }

        if self.options.nup > 1 {
            match self.options.nup {
                2 => {
                    buf.append(&mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x4e\x55\x50\x3d\x32\r\n".to_vec());
                }
                4 => {
                    buf.append(&mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x4e\x55\x50\x3d\x34\r\n".to_vec());
                }
                _ => (),
            }
            match self.options.nuppageorder {
                0 => {
                    buf.append(
                        &mut b"\x50\x4a\x4c\x20\x53\x45\x54\x20\x4e\x55\x50\x50\x41\x47\x45\x4f\x52\x44\x45\x52\x3d\x52\x49\x47\x48\x54\x54\x48\x45\x4e\x44\x4f\x57\x4e\r\n"
                            .to_vec(),
                    );
                }
                1 => {
                    buf.append(
                        &mut b"\x50\x4a\x4c\x20\x53\x45\x54\x20\x4e\x55\x50\x50\x41\x47\x45\x4f\x52\x44\x45\x52\x3d\x44\x4f\x57\x4e\x54\x48\x45\x4e\x52\x49\x47\x48\x54\r\n"
                            .to_vec(),
                    );
                }
                2 => {
                    buf.append(
                        &mut b"\x50\x4a\x4c\x20\x53\x45\x54\x20\x4e\x55\x50\x50\x41\x47\x45\x4f\x52\x44\x45\x52\x3d\x4c\x45\x46\x54\x54\x48\x45\x4e\x44\x4f\x57\x4e\r\n"
                            .to_vec(),
                    );
                }
                3 => {
                    buf.append(
                        &mut b"\x50\x4a\x4c\x20\x53\x45\x54\x20\x4e\x55\x50\x50\x41\x47\x45\x4f\x52\x44\x45\x52\x3d\x44\x4f\x57\x4e\x54\x48\x45\x4e\x4c\x45\x46\x54\r\n"
                            .to_vec(),
                    );
                }
                _ => (),
            }
        }
        if self.options.range != "" && page_range_is_valid(&self.options.range) {
            buf.append(
                &mut format!(
                    "\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x50\x52\x49\x4e\x54\x50\x41\x47\x45\x53\x3d\"{}\"\r\n",
                    self.options.range
                ).as_bytes()
                    .to_owned(),
            );
        }
        //set defaults for sanity
        buf.append(&mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x41\x55\x54\x4f\x54\x52\x41\x59\x43\x48\x41\x4e\x47\x45\x20\x4f\x4e\r\n".to_vec());
        buf.append(
            &mut b"\x40\x50\x4a\x4c\x20\x53\x45\x54\x20\x4d\x45\x44\x49\x41\x54\x59\x50\x45\x3d\x50\x4c\x41\x49\x4e\x4e\x4f\x52\x45\x43\x59\x43\x4c\x45\x44\r\n".to_vec(),
        );

        buf.append(&mut b"\x40\x50\x4a\x4c\x20\x45\x4e\x54\x45\x52\x20\x4c\x41\x4e\x47\x55\x41\x47\x45\x3d\x50\x44\x46\r\n".to_vec());

        buf
    }
}
