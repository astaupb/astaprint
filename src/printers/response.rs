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
use mysql::printers::Printer;
use printers::queue::get::WorkerTaskResponse;
use snmp::{
    tool::*,
    CounterValues,
    StatusValues,
};
#[derive(Serialize, Debug, Clone)]
pub struct PrinterResponse
{
    pub id: u32,
    pub hostname: String,
    pub ip: String,
    pub community: String,
    pub mac: String,
    pub device_id: u32,
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
    pub description: String,
    pub watch_toner: bool,
    pub watch_tray1: bool,
    pub watch_tray2: bool,
    pub watch_tray3: bool,
    pub watch_interval: u32,
    pub last_watched: i64,
    pub status: Option<StatusValues>,
    pub counter: Option<CounterValues>,
    pub queue: Option<WorkerTaskResponse>,
}

impl<'a> From<&'a Printer> for PrinterResponse
{
    fn from(printer: &'a Printer) -> PrinterResponse
    {
        PrinterResponse {
            id: printer.id,
            hostname: printer.hostname.clone(),
            ip: printer.ip.clone(),
            community: printer.community.clone(),
            mac: printer.mac.clone(),
            device_id: printer.device_id,
            location: printer.location.clone(),
            has_a3: printer.has_a3,
            coin_operated: printer.coin_operated,
            description: printer.description.clone(),
            watch_toner: printer.watch_toner,
            watch_tray1: printer.watch_tray1,
            watch_tray2: printer.watch_tray2,
            watch_tray3: printer.watch_tray3,
            watch_interval: printer.watch_interval,
            last_watched: printer.last_watched.timestamp(),
            status: None,
            counter: None,
            queue: None,
        }
    }
}

impl From<Printer> for PrinterResponse
{
    fn from(printer: Printer) -> PrinterResponse
    {
        let status = Some(status(&printer.ip).unwrap_or_default());

        let counter = Some(counter(&printer.ip).unwrap_or_default());

        PrinterResponse {
            id: printer.id,
            hostname: printer.hostname,
            ip: printer.ip,
            community: printer.community,
            mac: printer.mac,
            device_id: printer.device_id,
            location: printer.location,
            has_a3: printer.has_a3,
            coin_operated: printer.coin_operated,
            description: printer.description,
            watch_toner: printer.watch_toner,
            watch_tray1: printer.watch_tray1,
            watch_tray2: printer.watch_tray2,
            watch_tray3: printer.watch_tray3,
            watch_interval: printer.watch_interval,
            last_watched: printer.last_watched.timestamp(),
            status,
            counter,
            queue: None,
        }
    }
}
