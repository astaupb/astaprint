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

pub mod http;

use snmp::{
    CounterValues,
    StatusValues,
};

use model::task::worker::WorkerTask;

use mysql::printers::{
    insert::PrinterInsert,
    Printer,
};

/// Representation of a task blocking a printer displayed to an admin
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkerTaskResponse
{
    user_id: u32,
    uid: String,
}

impl<'a> From<&'a WorkerTask> for WorkerTaskResponse
{
    fn from(task: &WorkerTask) -> WorkerTaskResponse
    {
        WorkerTaskResponse {
            user_id: task.user_id,
            uid: hex::encode(&task.uid[..]),
        }
    }
}

/// Represenation of a printer displayed to an admin
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

/// POST Body needed to create a new admin
#[derive(Deserialize, Debug, Clone)]
pub struct PrinterCreate
{
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

impl From<PrinterCreate> for PrinterInsert
{
    fn from(post: PrinterCreate) -> PrinterInsert
    {
        PrinterInsert {
            hostname: post.hostname,
            ip: post.ip,
            community: post.community,
            mac: post.mac,
            device_id: post.device_id,
            location: post.location,
            has_a3: post.has_a3,
            coin_operated: post.coin_operated,
            description: post.description,
        }
    }
}
