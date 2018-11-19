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

extern crate astaprint;
extern crate logger;
extern crate r2d2_redis;
extern crate taskqueue;

use std::{
    collections::HashMap,
    env,
    sync::mpsc,
    thread,
};

use r2d2_redis::{
    r2d2::Pool,
    RedisConnectionManager,
};

use logger::Logger;
use taskqueue::{
    create_pool,
    TaskQueue,
};

use astaprint::printers::{
    select_device_ids,
    queue::task::{
        work,
        WorkerCommand,
        WorkerTask,
    },
    snmp::PrinterInterface,
};

use astaprint::establish_connection;

fn spawn_worker(device_id: u16, pool: Pool<RedisConnectionManager>) -> thread::JoinHandle<()>
{
    let interface = PrinterInterface::from_device_id(device_id);
    let name = format!("worker::{}", device_id);
    let taskqueue: TaskQueue<HashMap<Vec<u8>, WorkerTask>, PrinterInterface> =
        TaskQueue::new(&name, interface, pool);
    thread::spawn(move || {
        info!("{} listening", device_id);
        taskqueue
            .listen(|map, interface| {
                for (uid, task) in map {
                    let (sender, receiver): (
                        mpsc::Sender<WorkerCommand>,
                        mpsc::Receiver<WorkerCommand>,
                    ) = mpsc::channel();
                    work(receiver, uid, task, interface.clone());
                    sender.send(WorkerCommand::Print).expect("sending Print Command");
                }
            })
            .expect("processing Worker Tasks");
    })
}
fn main()
{
    Logger::init("worker").expect("initializing Logger");

    let url = env::var("ASTAPRINT_REDIS_URL").expect("reading redis url from environment");

    let pool = create_pool(&url);

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for id in select_device_ids(&establish_connection()) {
        handles.push(spawn_worker(id, pool.clone()));
    }

    for handle in handles {
        handle.join().expect("joining thread");
    }

}
