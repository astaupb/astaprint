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
use sodium::random_bytes;

use std::{
    fs::{
        remove_file,
        File,
    },
    io::{
        self,
        Read,
        Write,
    },
};

#[derive(Clone, Debug)]
pub struct TmpFile;

impl TmpFile
{
    pub fn create(data: &[u8]) -> io::Result<String>
    {
        let uid = random_bytes(20);

        let path = format!("/tmp/{}", hex::encode(&uid[..]));

        let mut file = File::create(&path)?;

        file.write_all(&data[..])?;

        file.sync_all()?;

        Ok(path)
    }

    pub fn remove(path: &str) -> io::Result<Vec<u8>>
    {
        let mut file = File::open(path)?;

        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf)?;

        file.sync_all()?;

        remove_file(path)?;

        Ok(buf)
    }

    pub fn read(path: &str) -> io::Result<Vec<u8>>
    {
        let mut file = File::open(path)?;

        let mut buf: Vec<u8> = Vec::new();

        file.read_to_end(&mut buf)?;

        file.sync_all()?;

        Ok(buf)
    }
}
