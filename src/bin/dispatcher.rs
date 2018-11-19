/// AStAPrint - Dispatcher
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
extern crate astaprint;
extern crate logger;
extern crate taskqueue;

use std::env;

use logger::Logger;

use taskqueue::{
    create_pool,
    TaskQueue,
};

use astaprint::jobs::{
    pdf::dispatch,
    task::DispatcherTask,
};

fn main()
{
    let url = env::var("ASTAPRINT_REDIS_URL").expect("reading redis url from environment");

    let pool = create_pool(&url);

    let taskqueue: TaskQueue<DispatcherTask> = TaskQueue::new("dispatcher", pool);

    Logger::init("dispatcher")
        .expect("initialising logger");

    taskqueue
        .listen(|task| {
            dispatch(task);
        })
        .unwrap_or_else(|e| println!("{}", e));
}
