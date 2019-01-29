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
extern crate bincode;
extern crate logger;
extern crate model;
extern crate redis;
extern crate threadpool;

extern crate mysql;
extern crate pdf;

use logger::Logger;

use threadpool::ThreadPool;

use redis::{
    get_redis_pool,
    queue::TaskQueue,
    store::Store,
    Redis,
};

use model::task::dispatcher::{
    DispatcherState,
    DispatcherTask,
};
extern crate astaprint;
use astaprint::jobs::queue::dispatch;

use mysql::get_mysql_pool;

fn main()
{
    let redis_store = Store::from(get_redis_pool(20, Redis::Store));

    let redis_pool = get_redis_pool(20, Redis::Dispatcher);

    let mysql_pool = get_mysql_pool(20);

    let thread_pool = ThreadPool::new(20);

    let state = DispatcherState {
        mysql_pool,
        redis_store,
        thread_pool,
    };

    let taskqueue: TaskQueue<DispatcherTask, DispatcherState, ()> =
        TaskQueue::new("dispatcher", state, redis_pool);

    Logger::init().expect("initialising logger");

    info!("listening");

    taskqueue.listen(|task, state, _client| {
        dispatch(task, state.clone(), state.thread_pool);
    });
}
