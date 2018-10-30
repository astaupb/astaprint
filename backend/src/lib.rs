#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

/// AStAPrint-Backend
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

#[macro_use]

extern crate log;

extern crate base64;
extern crate bigdecimal;
extern crate chrono;
extern crate diesel;
extern crate r2d2;
extern crate serde;
extern crate serde_json;

#[macro_use]

extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;

extern crate json_receiver;

extern crate astaprint;

pub mod crypto;
pub mod environment;
pub mod guards;
pub mod response;
pub mod routes;
