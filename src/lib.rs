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
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate hex;

extern crate diesel;

extern crate maxminddb;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

extern crate r2d2_redis;
extern crate redis;

extern crate legacy;
extern crate model;
extern crate mysql;
extern crate pdf;
extern crate snmp;

extern crate threadpool;

extern crate base64;
extern crate chrono;

extern crate lpr;

extern crate sodium;

extern crate poppler;

// routes
pub mod admin;
pub mod jobs;
pub mod journal;
pub mod printers;
pub mod user;
