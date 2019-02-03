// AStAPrint
// Copyright (C) 2018, 2019 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//
// This file is part of AStAPrint
//
// AStAPrint is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::io::Read;

use rocket::{
    data::{
        self,
        FromDataSimple,
    },
    http::{
        ContentType,
        Status,
    },
    Data,
    Outcome,
    Request,
};

pub const CHUNK_SIZE: usize = 4096;

#[derive(Debug, Clone)]
pub struct PdfBody
{
    pub bytes: Vec<u8>,
}

impl FromDataSimple for PdfBody
{
    type Error = String;

    fn from_data(
        req: &Request,
        data: Data,
    ) -> data::Outcome<Self, String>
    {
        if req.content_type() != Some(&ContentType::new("application", "pdf")) {
            return Outcome::Forward(data)
        }
        {
            let peek = data.peek();

            if !&String::from_utf8_lossy(&peek).contains("%PDF-1") {
                return Outcome::Failure((
                    Status::BadRequest,
                    "could not find %PDF-1 while peeking".to_string(),
                ))
            }
        }

        let mut bytes = Vec::<u8>::with_capacity(8192);

        let mut chunk = [0; CHUNK_SIZE];

        let mut stream = data.open();

        loop {
            let bytes_read = match stream.read(&mut chunk) {
                Ok(size) => size,
                Err(e) => {
                    return Outcome::Failure((Status::InternalServerError, format!("{:?}", e)))
                },
            };
            if bytes_read == 0 {
                break
            }
            bytes.extend_from_slice(&mut chunk[.. bytes_read]);
        }

        return Outcome::Success(PdfBody {
            bytes,
        })
    }
}
