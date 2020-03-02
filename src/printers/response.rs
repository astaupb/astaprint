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

#[derive(Serialize, Debug, Clone)]
pub struct UserPrinterResponse
{
    pub device_id: u32,
    pub location: String,
    pub has_a3: bool,
    pub coin_operated: bool,
}

impl<'a> From<&'a Printer> for UserPrinterResponse
{
    fn from(printer: &'a Printer) -> UserPrinterResponse
    {
        UserPrinterResponse {
            device_id: printer.device_id,
            location: printer.location.clone(),
            has_a3: printer.has_a3,
            coin_operated: printer.coin_operated,
        }
    }
}
