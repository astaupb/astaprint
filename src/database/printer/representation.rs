/// AStAPrint-Database - Printer Representation
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
use super::schema::*;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "printer"]

pub struct Printer
{
    pub id: u16,
    pub hostname: String,
    pub ip: String,
    pub community: String,
    pub mac: String,
    pub master: Option<String>,
    pub device_id: u16,
    pub model_id: u16,
    pub location: String,
    pub description: String,
}

joinable!(printer -> model (model_id));

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "model"]

pub struct Model
{
    pub id: u16,
    pub counter_id: u16,
    pub name: String,
}

joinable!(model -> counter (counter_id));

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "counter"]

pub struct Counter
{
    pub id: u16,
    pub total: String,
    pub print_black: String,
    pub print_color: Option<String>,
    pub copy_black: String,
    pub copy_color: Option<String>,
    pub description: String,
}

joinable!(model -> queue_ctl (queue_ctl_id));

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "queue_ctl"]

pub struct QueueCtl
{
    pub id: u16,
    pub oid: String,
    pub cancel: i32,
    pub clear: i32,
}

joinable!(model -> energy_ctl (energy_ctl_id));

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "energy_ctl"]

pub struct EnergyCtl
{
    pub id: u16,
    pub oid: String,
    pub wake: i32,
    pub sleep: i32,
}
