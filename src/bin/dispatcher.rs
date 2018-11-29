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
extern crate redis;
extern crate threadpool;

use std::env;

use logger::Logger;

use threadpool::ThreadPool;

use redis::queue::TaskQueue;

use astaprint::{
    jobs::{
        pdf::dispatch,
        task::{
            DispatcherState,
            DispatcherTask,
        },
    },
    pool::{
        create_mysql_pool,
        create_redis_pool,
    },
};

fn main()
{
    let redis_url = env::var("ASTAPRINT_REDIS_URL").expect("reading redis url from environment");

    let redis_pool = create_redis_pool(&redis_url, 10);

    let mysql_url = env::var("ASTAPRINT_DATABASE_URL").expect("reading database url from environment");

    let mysql_pool = create_mysql_pool(&mysql_url, 10);

    let thread_pool = ThreadPool::new(20);

    let state = DispatcherState {
        mysql_pool,
    };
    let taskqueue: TaskQueue<DispatcherTask, DispatcherState> =
        TaskQueue::new("dispatcher", state, redis_pool, thread_pool);

    Logger::init("dispatcher").expect("initialising logger");

    info!("listening");

    taskqueue.listen(|task, state| {
        dispatch(task, state.mysql_pool);
    });
}
