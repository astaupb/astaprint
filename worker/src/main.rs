/// AStAPrint-Worker - main.rs
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

extern crate inotify;
extern crate serde_json;

#[macro_use]

extern crate serde_derive;

extern crate json_receiver;
extern crate lpr;
extern crate mpsc_queue;

extern crate astaprint;
extern crate worker;

use lpr::LprConnection;

use json_receiver::JsonReceiver;

use mpsc_queue::{
    Queue,
    QueueHandler,
    Unique,
};

use std::{
    env,
    sync::mpsc,
    thread,
    time,
};

use worker::{
    accounting::Accounting,
    snmp::{
        session::SnmpSession,
        PrinterInterface,
    },
};

use astaprint::{
    job::{
        Job,
        JobData,
        PrintWorkerJSON,
    },
    logger::Logger,
};

#[derive(PartialEq)]

enum Command
{
    Print,
    Cancel,
}

#[derive(Serialize, Clone)]

struct QueueElement
{
    print: JobData,
    #[serde(skip)]
    cmd_sender: mpsc::Sender<Command>,
}

impl Unique for QueueElement
{
    fn uid(&self) -> &str
    {
        &self.print.uid
    }
}

fn spawn_queue_reader(mut queue: Queue<QueueElement>)
{
    thread::spawn(move || {
        loop {
            let next = queue.pop();

            next.cmd_sender.send(Command::Print).expect("sending print command to thread");
        }
    });
}

fn main()
{
    if env::args().count() != 2 {
        panic!("pass device_id as parameter");
    }

    let spooldir = env::var("ASTAPRINT_SPOOL_DIR").expect("reading spooldir from environemt");

    let device = env::args().nth(1).unwrap();

    let printer_interface: PrinterInterface =
        PrinterInterface::from_device_id(device.parse::<u16>().unwrap());

    let queue = Queue::new(&format!("{}/print/{}/queue.json", spooldir, device));

    let mut queue_handler = QueueHandler::from(&queue);

    spawn_queue_reader(queue);

    let receiver = JsonReceiver::<PrintWorkerJSON>::new(&format!("{}/print/{}", spooldir, device)).spawn();

    Logger::init(&format!("print/{}", device)).expect(&format!("initializing print/{} logger", device));

    info!("initialized");

    loop {
        let json = receiver.recv().expect("receiving JobJSON");

        match json {
            PrintWorkerJSON::print(json) => {
                let (tx, rx): (mpsc::Sender<Command>, mpsc::Receiver<Command>) = mpsc::channel();

                queue_handler.push(QueueElement {
                    print: json.clone(),
                    cmd_sender: tx,
                });

                work(rx, json, printer_interface.clone());
            },
            PrintWorkerJSON::cancel(json) => {
                if let Some(element) = queue_handler.remove_by_uid(&json.uid) {
                    element
                        .cmd_sender
                        .send(Command::Cancel)
                        .expect(&format!("sending cancel command to {}", &json.uid));
                }
            },
        }
    }
}

fn work(cmd_receiver: mpsc::Receiver<Command>, json: JobData, mut interface: PrinterInterface)
{
    thread::Builder::new()
        .name(json.uid[..16].to_string())
        .spawn(move || {
            info!("{} print thread spawned for {}", &json.uid[..16], &json.user_id);
            let job = Job::new(json);

            let mut buf: Vec<u8> = job.translate_for_printer();

            let snmp_session = SnmpSession::new(&interface.ip, &interface.community);

            let command = cmd_receiver.recv().expect("receiving command from queue reader");

            if command == Command::Cancel {
                info!("{} canceled printing", &job.data.user_id);
                return;
            }

            let mut accounting = Accounting::new(job.data.user_id, job.data.info.color);

            if accounting.not_enough_credit() {
                info!("not enough credit for one page, aborting");
                return;
            }

            let counter_base = snmp_session
                .get_counter_values(&mut interface.counter)
                .expect("reading base counter value");

            debug!("counter_base: {:?}", counter_base);

            let mut lpr_connection = LprConnection::new(&interface.ip, false);
            lpr_connection.print(&mut buf);

            let print_count = job.data.pages_to_print();

            // check energy status before initial waiting
            // 1 == ready
            let energy_stat = snmp_session
                .get_integer(&mut interface.energy_ctl.oid[..])
                .expect("getting energy status of device");
            debug!("energy stat: {}", &energy_stat);
            thread::sleep(time::Duration::from_millis(match energy_stat {
                1 => 2000,
                _ => {
                    match snmp_session
                        .set_integer(&mut interface.energy_ctl.oid[..], interface.energy_ctl.wake)
                    {
                        Ok(_) => 10000,
                        Err(_) => 12000,
                    }
                },
            }));
            let mut loop_count = 0;
            let mut last_value = counter_base.total;

            let completed = loop {
                thread::sleep(time::Duration::from_millis(20));
                let current = snmp_session
                    .get_counter_values(&mut interface.counter)
                    .expect("getting counter values");

                // reset loop count if another page is printed
                if current.total > last_value {
                    debug!("current: {:?}", current);
                    last_value = current.total;
                    loop_count = 0;
                    accounting.set_value(&(current.clone() - counter_base.clone()));
                } else {
                    loop_count += 1;
                }

                if (current.total - counter_base.total) == print_count as u64 {
                    debug!("current: {:?}", current);
                    break true;
                }

                if accounting.not_enough_credit() {
                    debug!("current: {:?}", current);
                    info!("{} {} no credit left, clearing jobqueue", &job.data.uid[..16], job.data.user_id);
                    break false;
                }

                if loop_count > 420 {
                    debug!("current: {:?}", current);
                    warn!("{} {} timeout", &job.data.uid[..16], job.data.user_id);
                    break false;
                }

                // check for cancel command
                if let Ok(Command::Cancel) = cmd_receiver.try_recv() {
                    info!("{} {} canceled printing in progress", &job.data.uid[..16], &job.data.user_id);
                    break false;
                }
            };

            // clear jobqueue on every outcome in case printer wants to print more than expected
            snmp_session
                .set_integer(&mut interface.queue_ctl.oid[..], interface.queue_ctl.clear)
                .expect("clearing jobqueue");

            accounting.finish();

            debug!("{} keep: {} - completed: {}", &job.data.uid[..16], job.data.options.keep, completed);
            if !job.data.options.keep && completed {
                job.files.clean_up(job.data.info.pagecount);
            }

            info!("{} finished", &job.data.uid[..16]);
        })
        .expect("spawning print thread");
}
