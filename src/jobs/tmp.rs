/// AStAPrint - Jobs - Temporary File
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
use astacrypto::random_bytes;

use jobs::uid::UID;

use std::{
    fs::File,
    io::{
        Read,
        Write,
    },
};

#[derive(Clone, Debug)]
pub struct TemporaryFile
{
    pub path: String,
}

impl TemporaryFile
{
    pub fn new(data: &[u8]) -> TemporaryFile
    {
        let uid = UID::from(random_bytes(20));

        let path = format!("/tmp/{:x}", uid);

        let mut file = File::create(&path).expect("creating temporary file");

        file.write_all(data).expect("writing data to file");

        TemporaryFile {
            path,
        }
    }

    pub fn close(self) -> Vec<u8>
    {
        let mut file = File::open(&self.path).expect("opening temporary file");

        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf).expect("reading file into buffer");

        buf
    }
}
