pub mod representation;
/// AStAPrint-Database - Printer
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
pub mod schema;

use self::{
    representation::*,
    schema::*,
};

use diesel::prelude::*;

use super::establish_connection;

pub fn select_printer_interface_information(
    device_id: u16,
) -> (Counter, QueueCtl, EnergyCtl, String, String)
{
    let result: (Counter, QueueCtl, EnergyCtl, String, String) = counter::table
        .inner_join(
            model::table
                .inner_join(printer::table)
                .inner_join(energy_ctl::table)
                .inner_join(queue_ctl::table),
        )
        .select((
            counter::all_columns,
            queue_ctl::all_columns,
            energy_ctl::all_columns,
            printer::community,
            printer::ip,
        ))
        .filter(printer::device_id.eq(device_id))
        .first(&establish_connection())
        .expect("fetching printer interface information");

    result
}

pub fn select_device_ids() -> Vec<u16>
{
    printer::table.select(printer::device_id).load(&establish_connection()).expect("fetching device ids")
}
