/// AStAPrint PDF - document.rs
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
use std::fmt;

impl fmt::LowerHex for UID
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        for c in &self.bytes {
            write!(f, "{:x}", c)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UID
{
    bytes: Vec<u8>,
}

impl From<Vec<u8>> for UID
{
    fn from(bytes: Vec<u8>) -> UID
    {
        UID {
            bytes,
        }
    }
}
