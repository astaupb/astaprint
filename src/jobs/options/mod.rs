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
pub mod get;
pub mod put;

use model::job::options::JobOptions;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value
{
    S(String),
    I(u16),
    B(bool),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobOptionsUpdate
{
    pub color: Option<bool>,
    pub duplex: Option<u8>,
    pub copies: Option<u16>,
    pub collate: Option<bool>,
    pub bypass: Option<bool>,
    pub keep: Option<bool>,
    pub a3: Option<bool>,
    pub nup: Option<u8>,
    pub nuppageorder: Option<u8>,
    pub range: Option<String>,
}

pub trait Update
{
    fn merge(
        &mut self,
        update: JobOptionsUpdate,
    );
}

impl Update for JobOptions
{
    fn merge(
        &mut self,
        update: JobOptionsUpdate,
    )
    {
        if let Some(color) = update.color {
            self.color = color; 
        }

        if let Some(duplex) = update.duplex {
            self.duplex = duplex;
        }

        if let Some(copies) = update.copies {
            self.copies = copies;
        }

        if let Some(collate) = update.collate {
            self.collate = collate;
        }

        if let Some(bypass) = update.bypass {
            self.bypass = bypass;
        }

        if let Some(keep) = update.keep {
            self.keep = keep;
        }

        if let Some(a3) = update.a3 {
            self.a3 = a3;
        }

        if let Some(nup) = update.nup {
            self.nup = nup;
        }

        if let Some(nuppageorder) = update.nuppageorder {
            self.nuppageorder = nuppageorder;
        }

        if let Some(range) = update.range {
            self.range = range;
        }
    }
}
