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
pub mod accounting;
pub mod delete;
pub mod get;
pub mod queue;
pub mod response;
pub mod update;

use jobs::options::JobOptionsUpdate;

use model::task::worker::{
    WorkerCommand,
    WorkerTask,
};

use redis::queue::TaskQueueClient;
use std::collections::{
    hash_map::RandomState,
    HashMap,
};

pub type PrinterQueue = TaskQueueClient<WorkerTask, WorkerCommand<Option<JobOptionsUpdate>>>;
pub type PrinterQueues = HashMap<u32, PrinterQueue, RandomState>;
