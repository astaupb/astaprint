/// AStAPrint-Common
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

extern crate log;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate chrono;

extern crate cairo;
extern crate poppler;

extern crate astaprint_database;

pub mod accounting;
pub mod file;
pub mod job;
pub mod lock;
pub mod logger;
pub mod pagerange;
pub mod pdf;

pub mod snmp;
pub mod subprocesses;
