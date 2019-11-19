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

pub fn insert_into_printers(
    hostname: String,
    ip: String,
    community: String,
    mac: String,
    device_id: u32,
    location: String,
    has_a3: bool,
    coin_operated: bool,
    description: String,
    connection: &MysqlConnection,
) -> QueryResult<usize>
{
    insert_into(printers::table)
        .values((
           printers::hostname.eq(hostname),
           printers::ip.eq(ip),
           printers::community.eq(community),
           printers::mac.eq(mac),
           printers::device_id.eq(device_id),
           printers::location.eq(location),
           printers::has_a3.eq(has_a3),
           printers::coin_operated.eq(coin_operated),
           printers::description.eq(description),
        ))
        .execute(connection)
}
