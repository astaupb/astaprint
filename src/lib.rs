#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
/// AStAPrint - lib.rs
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
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

#[macro_use]
extern crate diesel;

extern crate rocket;
extern crate rocket_contrib;

extern crate redis;

extern crate base64;
extern crate bigdecimal;
extern crate chrono;
extern crate sha2;

extern crate cairo;
extern crate poppler;

extern crate lpr;

extern crate astacrypto;
extern crate taskqueue;

// routes
pub mod jobs;
pub mod journal;
pub mod manager;
pub mod printers;
pub mod user;

use diesel::prelude::*;
use std::env;
pub fn establish_connection() -> MysqlConnection
{
    let url = env::var("ASTAPRINT_DATABASE_URL").expect("reading ASTAPRINT_DATABASE_URL from environment");

    MysqlConnection::establish(&url).expect("establishing MysqlConnection")
}
