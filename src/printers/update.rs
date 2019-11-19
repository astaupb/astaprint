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

#[derive(Deserialize, Debug, Clone)]
pub struct PrinterUpdate
{
    pub hostname: Option<String>,
    pub ip: Option<String>,
    pub community: Option<String>,
    pub mac: Option<String>,
    pub device_id: Option<u32>,
    pub location: Option<String>,
    pub has_a3: Option<bool>,
    pub coin_operated: Option<bool>,
    pub description: Option<String>,
    pub watch_toner: Option<bool>,
    pub watch_tray1: Option<bool>,
    pub watch_tray2: Option<bool>,
    pub watch_tray3: Option<bool>,
    pub watch_interval: Option<u32>,
}

impl PrinterUpdate
{
    pub fn update(&self, mut printer: Printer) -> Printer
    {
        if let Some(hostname) = &self.hostname {
            printer.hostname = hostname.clone();
        }
        if let Some(ip) = &self.ip {
            printer.ip = ip.clone();
        }
        if let Some(community) = &self.community {
            printer.community = community.clone();
        }
        if let Some(mac) = &self.mac {
            printer.mac = mac.clone();
        }
        if let Some(device_id) = self.device_id {
            printer.device_id = device_id;
        }
        if let Some(location) = &self.location {
            printer.location = location.clone();
        }
        if let Some(has_a3) = self.has_a3 {
            printer.has_a3 = has_a3;
        }
        if let Some(coin_operated) = self.coin_operated {
            printer.coin_operated = coin_operated;
        }
        if let Some(description) = &self.description {
            printer.description = description.clone();
        }
        if let Some(watch_toner) = self.watch_toner {
            printer.watch_toner = watch_toner;
        }
        if let Some(watch_tray1) = self.watch_tray1 {
            printer.watch_tray1 = watch_tray1;
        }
        if let Some(watch_tray2) = self.watch_tray2 {
            printer.watch_tray2 = watch_tray2;
        }
        if let Some(watch_tray3) = self.watch_tray3 {
            printer.watch_tray3 = watch_tray3;
        }
        if let Some(watch_interval) = self.watch_interval {
            printer.watch_interval = watch_interval;
        }
        printer
    }
}
