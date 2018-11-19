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
extern crate astaprint;
extern crate logger;
extern crate taskqueue;
extern crate r2d2_redis;

use std::{
    collections::HashMap,
    sync::mpsc,
    env,
    thread,
};

use r2d2_redis::{
    r2d2::Pool,
    RedisConnectionManager,
};

use taskqueue::{
    TaskQueue,
    create_pool,
};
use logger::Logger;

use astaprint::printers::{
    queue::task::{
        work, WorkerTask, WorkerCommand,
    },
    snmp::PrinterInterface,
};

fn spawn_worker(device_id: u16, pool: Pool<RedisConnectionManager>)
{
    let interface = PrinterInterface::from_device_id(device_id);
    let taskqueue: TaskQueue<HashMap<Vec<u8>, WorkerTask>, PrinterInterface> = TaskQueue::new(&format!("worker::{}", device_id), interface, pool);
    thread::spawn(move || {
        taskqueue.listen(|map, interface| {
            for (uid, task) in map {
                let (sender, receiver): (mpsc::Sender<WorkerCommand>, mpsc::Receiver<WorkerCommand>) = mpsc::channel();
                work(receiver, uid, task, interface.clone());
                sender.send(WorkerCommand::Print)
                    .expect("sending Print Command");
            }
        }).expect("processing Worker Tasks"); 
    })
    .join().expect("joining worker thread");
}
fn main()
{
    let device_id = env::args().nth(1)
        .expect("device_id as first argument");

    let device_id: u16 = device_id.parse()
        .expect("parsing device_id from String");

    Logger::init("worker")
        .expect("initializing Logger");

    let url = env::var("ASTAPRINT_REDIS_URL")
        .expect("reading redis url from environment");

    let pool = create_pool(&url);

    spawn_worker(device_id, pool.clone());

}
