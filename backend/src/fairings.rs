/// AStAPrint-Backend - Fairings
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
    fairing::{Fairing, Info, Kind},
    http::hyper::header::Connection,
    Response,
    Request,
};

#[derive(Default)]
pub struct KeepAlive;

impl Fairing for KeepAlive {
    fn info(&self) -> Info {
        Info {
            name: "Keep Alive Header",
            kind: Kind::Response,
        }
    }
    fn on_response(&self, request: &Request, response: &mut Response) {
        for header in request.headers().iter() {
            println!("{:?}", header);
        }
        response.set_header(Connection::keep_alive());
        for header in response.headers().iter() {
            println!("{:?}", header);
        }

    }
}
