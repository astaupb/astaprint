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
extern crate logger;
extern crate redis;
extern crate threadpool;
extern crate bincode;

extern crate pdf;
extern crate mysql;
extern crate astaprint;


use std::env;

use logger::Logger;

use threadpool::ThreadPool;

use redis::{
    create_redis_pool,
    queue::TaskQueue,
    store::Store,
};

use astaprint::{
    jobs::{
        info::JobInfo,
        options::JobOptions,
        task::{
            DispatcherState,
            DispatcherTask,
        },
    },
};
use mysql::{
    create_mysql_pool,
    jobs::insert::insert_into_jobs,
};

use pdf::sanitize;

pub fn dispatch(task: DispatcherTask, state: DispatcherState)
{
    let hex_uid = hex::encode(&task.uid[..]);
    info!("{} {} started", task.user_id, &hex_uid[..8]);

    let data = state.redis_store.get(task.uid).expect("getting file from store");

    let result = sanitize(data);

    let connection = state.mysql_pool.get().expect("getting mysql connection from pool");

    let info: Vec<u8> = bincode::serialize(&JobInfo {
        filename: task.filename,
        title: result.title,
        pagecount: result.pagecount,
        colored: result.colored,
        a3: result.a3,
    }).expect("serializing JobInfo");

    let options: Vec<u8> = bincode::serialize(&JobOptions::default())
        .expect("serializing JobOptions");

    insert_into_jobs(
        task.user_id,
        info,
        options,
        result.pdf,
        result.pdf_bw,
        result.preview_0,
        result.preview_1,
        result.preview_2,
        result.preview_3,
        &connection
    ).expect("inserting job into table");
}

fn main()
{
    let redis_url = env::var("ASTAPRINT_REDIS_URL")
        .expect("reading redis url from environment");

    let redis_pool = create_redis_pool(&redis_url, 10);

    let redis_store = Store::from(redis_pool.clone());

    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading mysql url from environment");
    
    let mysql_pool = create_mysql_pool(&mysql_url, 10);

    let thread_pool = ThreadPool::new(20);

    let state = DispatcherState {
        mysql_pool,
        redis_store,
    };
    let taskqueue: TaskQueue<DispatcherTask, DispatcherState> =
        TaskQueue::new("dispatcher", state, redis_pool, thread_pool);

    Logger::init().expect("initialising logger");

    info!("listening");

    taskqueue.listen(|task, state| {
        dispatch(task, state);
    });
}
