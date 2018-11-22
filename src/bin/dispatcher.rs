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
#[macro_use]
extern crate log;
extern crate astaprint;
extern crate logger;
extern crate taskqueue;
extern crate threadpool;

use std::{
    env,
};

use logger::Logger;

use threadpool::ThreadPool;

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

    let redis_pool = create_pool(&url);

    let thread_pool = ThreadPool::new(20);

    let taskqueue: TaskQueue<DispatcherTask, ThreadPool> = TaskQueue::new("dispatcher", thread_pool, redis_pool);

    Logger::init("dispatcher").expect("initialising logger");

    info!("listening");

    taskqueue
        .listen(|task, thread_pool| {
            thread_pool.execute(move || {
                dispatch(task);
            });
        })
        .unwrap_or_else(|e| println!("{}", e));
}
