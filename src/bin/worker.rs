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
#[macro_use]
extern crate log;
extern crate diesel;

extern crate logger;
extern crate r2d2_redis;
extern crate redis;

extern crate model;
extern crate mysql;
extern crate snmp;

extern crate astaprint;

use std::thread;

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
    get_redis_pool,
    queue::TaskQueue,
    Redis,
};

use mysql::{
    get_mysql_pool,
    printers::select::{
        select_device_ids,
        select_ip_by_device_id,
    },
};

use model::{
    job::options::update::JobOptionsUpdate,
    ppd::PPD,
    task::worker::{
        WorkerCommand,
        WorkerState,
        WorkerTask,
    },
};

use astaprint::printers::queue::work;

fn spawn_worker(
    device_id: u32,
    ip: String,
    ppd: PPD,
    redis_pool: RedisPool<RedisConnectionManager>,
    mysql_pool: MysqlPool<ConnectionManager<MysqlConnection>>,
) -> thread::JoinHandle<()>
{
    let name = format!("worker::{}", device_id);

    let taskqueue: TaskQueue<WorkerTask, WorkerState, WorkerCommand<Option<JobOptionsUpdate>>> =
        TaskQueue::new(
            &name,
            WorkerState {
                device_id,
                ip,
                ppd,
                mysql_pool,
                redis_pool: redis_pool.clone(),
            },
            redis_pool,
        );

    thread::spawn(move || {
        info!("{} listening", device_id);
        taskqueue.listen(|task, state, client| {
            let c = client.clone();
            let t = task.clone();

            let join_result = thread::spawn(move || {
                work(task, state, client);
            })
            .join();

            if let Err(e) = join_result {
                error!("{:?}", e);
            }

            c.finish(&t).expect("removing task from queue");
        });
    })
}
fn main()
{
    Logger::init().expect("initializing Logger");

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    let mysql_pool = get_mysql_pool(20);

    let connection = mysql_pool.get().expect("getting mysql connection from pool");

    let ppd = PPD::new_from_file("./Ricoh-MP_C4504ex-PDF-Ricoh.ppd").expect("creating PPD");

    for id in select_device_ids(&connection).expect("selecting device ids") {
        let redis_pool = get_redis_pool(32, Redis::Worker);

        let ip = select_ip_by_device_id(id, &connection).expect("selecting ip by device_id");

        handles.push(spawn_worker(id, ip, ppd.clone(), redis_pool, mysql_pool.clone()));
    }

    for handle in handles {
        handle.join().expect("joining thread");
    }
}
