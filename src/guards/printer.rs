/// AStAPrint-Backend - Printers
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
use rocket::{
    http::Status,
    request::{
        self,
        FromRequest,
    },
    Outcome,
    Request,
    State,
};

use diesel::{
    insert_into,
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use astaprint::database::user::schema::*;

pub struct Printer {
    id: u16,
    queue: String,
}

impl FromRequest for Printer {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Printer, ()> {
        let  ff 
    }
}
