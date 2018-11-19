/// AStAPrint-Worker
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
pub mod counter;
pub mod session;

use diesel::prelude::*;

use self::counter::CounterOids;
use establish_connection;
use printers::*;

#[derive(Debug, Clone)]

pub struct PrinterInterface
{
    pub ip: String,
    pub community: String,
    pub counter: CounterOids,
    pub queue_ctl: QueueControl,
    pub energy_ctl: EnergyControl,
}

impl PrinterInterface
{
    pub fn from_device_id(device_id: u16) -> PrinterInterface
    {
        let (row, queue_ctl, energy_ctl, community, ip): (
            Counter,
            QueueCtl,
            EnergyCtl,
            String,
            String,
        ) = select_printer_interface_information(device_id);

        PrinterInterface {
            ip,
            community,
            counter: CounterOids {
                total: vec_from_oid_str(&row.total),
                print_black: vec_from_oid_str(&row.print_black),
                print_color: row.print_color.map(|s| vec_from_oid_str(&s)),
                copy_black: vec_from_oid_str(&row.copy_black),
                copy_color: row.copy_color.map(|s| vec_from_oid_str(&s)),
            },
            queue_ctl: QueueControl::from(queue_ctl),
            energy_ctl: EnergyControl::from(energy_ctl),
        }
    }
}

#[derive(Debug, Clone)]

pub struct QueueControl
{
    pub oid: Vec<u64>,
    pub cancel: i32,
    pub clear: i32,
}

impl From<QueueCtl> for QueueControl
{
    fn from(queue_ctl: QueueCtl) -> Self
    {
        QueueControl {
            oid: vec_from_oid_str(&queue_ctl.oid),
            cancel: queue_ctl.cancel,
            clear: queue_ctl.clear,
        }
    }
}

#[derive(Debug, Clone)]

pub struct EnergyControl
{
    pub oid: Vec<u64>,
    pub wake: i32,
    pub sleep: i32,
}

impl From<EnergyCtl> for EnergyControl
{
    fn from(energy_ctl: EnergyCtl) -> Self
    {
        EnergyControl {
            oid: vec_from_oid_str(&energy_ctl.oid),
            wake: energy_ctl.wake,
            sleep: energy_ctl.sleep,
        }
    }
}

fn vec_from_oid_str(oid: &str) -> Vec<u64>
{
    use std::str::FromStr;

    oid.split(".").map(|x| u64::from_str(x).expect("converting oid str to u64")).collect()
}

pub fn select_printer_interface_information(
    device_id: u16,
) -> (Counter, QueueCtl, EnergyCtl, String, String)
{
    let result: (Counter, QueueCtl, EnergyCtl, String, String) = printer_counter::table
        .inner_join(
            printer_model::table
                .inner_join(printers::table)
                .inner_join(printer_energy_ctl::table)
                .inner_join(printer_queue_ctl::table),
        )
        .select((
            printer_counter::all_columns,
            printer_queue_ctl::all_columns,
            printer_energy_ctl::all_columns,
            printers::community,
            printers::ip,
        ))
        .filter(printers::device_id.eq(device_id))
        .first(&establish_connection())
        .expect("fetching printer interface information");

    result
}
