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
//! module containing all the operations on /printers
pub mod accounting;
pub mod http;
pub mod queue;

use std::collections::{
    hash_map::RandomState,
    HashMap,
};

use redis::queue::TaskQueueClient;

use model::{
    job::options::update::JobOptionsUpdate,
    task::worker::{
        WorkerCommand,
        WorkerTask,
    },
};

pub type PrinterQueue = TaskQueueClient<WorkerTask, WorkerCommand<Option<JobOptionsUpdate>>>;
pub type PrinterQueues = HashMap<u32, PrinterQueue, RandomState>;
