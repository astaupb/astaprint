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

use diesel::{
    prelude::*,
    insert_into,
};
use crate::schema::*;

#[derive(Deserialize, Debug, Clone)]
pub struct PrinterInsert {
    pub hostname: String,
    pub ip: String,
    pub community: String,
    pub mac: String,
    pub device_id: u32,
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
    pub description: String,
}

pub fn insert_into_printers(
    printer: PrinterInsert,
    connection: &MysqlConnection,
) -> QueryResult<usize>
{
    insert_into(printers::table)
        .values((
           printers::hostname.eq(printer.hostname),
           printers::ip.eq(printer.ip),
           printers::community.eq(printer.community),
           printers::mac.eq(printer.mac),
           printers::device_id.eq(printer.device_id),
           printers::location.eq(printer.location),
           printers::has_a3.eq(printer.has_a3),
           printers::coin_operated.eq(printer.coin_operated),
           printers::description.eq(printer.description),
        ))
        .execute(connection)
}
