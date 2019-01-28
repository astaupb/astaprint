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

extern crate logger;
extern crate r2d2_redis;
extern crate redis;

extern crate model;
extern crate mysql;
extern crate snmp;

use std::{
    thread,
};

use diesel::{
    mysql::MysqlConnection,
    r2d2::{
        ConnectionManager,
        Pool as MysqlPool,
    },
};

use r2d2_redis::{
    r2d2::Pool as RedisPool,
    RedisConnectionManager,
};

use logger::Logger;
use redis::{
    Redis,
    get_redis_pool,
    queue::TaskQueue,
};

use mysql::{
    get_mysql_pool,
    printers::select::select_device_ids,
};


use model::task::worker::{
    WorkerCommand,
    WorkerState,
    WorkerTask,
};
extern crate astaprint;
use astaprint::printers::queue::work;


use snmp::PrinterInterface;


fn spawn_worker(
    device_id: u32,
    redis_pool: RedisPool<RedisConnectionManager>,
    mysql_pool: MysqlPool<ConnectionManager<MysqlConnection>>,
) -> thread::JoinHandle<()>
{
    let connection = mysql_pool.get().expect("getting connection from pool");

    let printer_interface =
        PrinterInterface::from_device_id(device_id, &connection);

    let name = format!("worker::{}", device_id);

    let taskqueue: TaskQueue<WorkerTask, WorkerState, WorkerCommand> =
        TaskQueue::new(
            &name,
            WorkerState {
                printer_interface,
                mysql_pool,
                redis_pool: redis_pool.clone(),
            },
            redis_pool,
        );

    thread::spawn(move || {
        info!("{} listening", device_id);
        taskqueue.listen(|task, state, client| {
            work(task, state.clone(), client.clone());
        });
    })
}
fn main()
{
    Logger::init().expect("initializing Logger");

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    let mysql_pool = get_mysql_pool(20);

    let connection = mysql_pool.get().expect("getting mysql connection from pool");

    for id in select_device_ids(&connection).expect("selecting device ids") {
        let redis_pool = get_redis_pool(20, Redis::Worker);

        handles.push(spawn_worker(id, redis_pool, mysql_pool.clone()));
    }

    for handle in handles {
        handle.join().expect("joining thread");
    }
}
