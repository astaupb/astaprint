/// AStAPrint - Worker
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

extern crate diesel;
extern crate threadpool;

extern crate astaprint;
extern crate logger;
extern crate r2d2_redis;
extern crate redis;

use std::{
    env,
    thread,
};

use diesel::{
    mysql::MysqlConnection,
    r2d2::{
        ConnectionManager,
        Pool as MysqlPool,
    },
};

use threadpool::ThreadPool;

use r2d2_redis::{
    r2d2::Pool as RedisPool,
    RedisConnectionManager,
};

use logger::Logger;
use redis::queue::TaskQueue;

use astaprint::printers::{
    queue::task::{
        work,
        WorkerState,
        WorkerTask,
    },
    select_device_ids,
    snmp::PrinterInterface,
};

use astaprint::pool::{
    create_mysql_pool,
    create_redis_pool,
};


fn spawn_worker(
    device_id: u16,
    redis_pool: RedisPool<RedisConnectionManager>,
    mysql_pool: MysqlPool<ConnectionManager<MysqlConnection>>,
) -> thread::JoinHandle<()>
{
    let connection = mysql_pool.get().expect("getting connection from pool");

    let printer_interface = PrinterInterface::from_device_id(device_id, &connection);
    let name = format!("worker::{}", device_id);
    let thread_pool = ThreadPool::new(8);
    let taskqueue: TaskQueue<WorkerTask, WorkerState> = TaskQueue::new(
        &name,
        WorkerState {
            printer_interface,
            mysql_pool,
        },
        redis_pool,
        thread_pool,
    );

    thread::spawn(move || {
        info!("{} listening", device_id);
        taskqueue.listen(|task, state| {
            work(task, state.clone());
        });
    })
}
fn main()
{
    Logger::init("worker").expect("initializing Logger");

    let redis_url = env::var("ASTAPRINT_REDIS_URL").expect("reading redis url from environment");

    let mysql_url = env::var("ASTAPRINT_DATABASE_URL").expect("reading database url form environment");

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    let mysql_pool = create_mysql_pool(&mysql_url, 10);

    let connection = mysql_pool.get().expect("getting mysql connection from pool");

    for id in select_device_ids(&connection) {
        let redis_pool = create_redis_pool(&redis_url, 3);

        handles.push(spawn_worker(id, redis_pool, mysql_pool.clone()));
    }

    for handle in handles {
        handle.join().expect("joining thread");
    }
}
