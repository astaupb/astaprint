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
pub mod delete;
pub mod get;
pub mod post;

pub mod timeout;

use model::{
    job::Job,
    task::worker::{
        WorkerCommand,
        WorkerState,
        WorkerTask,
    },
};

use std::{
    collections::VecDeque,
    thread,
    time,
};

use pdf::{
    process::trim_pages,
    tmp::TmpFile,
};

use lpr::LprConnection;

use printers::{
    accounting::Accounting,
    queue::timeout::TimeOut,
};

use mysql::jobs::{
    delete::*,
    select::*,
};

use redis::queue::{
    CommandClient,
    TaskQueueClient,
};

use snmp::tool::*;

pub fn work(
    task: WorkerTask,
    state: WorkerState,
    client: TaskQueueClient<WorkerTask, WorkerCommand>,
)
{
    wake(&state.ip);
    let counter_base = match counter(&state.ip) {
        Ok(counter) => counter,
        Err(e) => {
            error!("get counter: {:?}", e);
            return
        },
    };
    debug!("counter_base: {:?}", counter_base);

    let hex_uid = hex::encode(&task.uid[..]);

    info!("{} user {} has locked device {}", &hex_uid[.. 8], task.user_id, state.device_id);

    let command_client = CommandClient::from((&client, &hex_uid[..]));

    let connection = match state.mysql_pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            error!("getting connection: {:?}", e);
            return
        },
    };

    let mut accounting =
        Accounting::new(task.user_id, state.device_id, counter_base, state.mysql_pool.clone());

    let mut timeout = TimeOut::new(60);
    let mut hungup = false;

    let mut to_print: VecDeque<u32> = VecDeque::new();
    let mut printing: Option<(u32, bool)> = None;

    let command_receiver = command_client.get_command_receiver();
    loop {
        if let Ok(command) = command_receiver.try_recv() {
            match command {
                WorkerCommand::Cancel => break,
                WorkerCommand::HeartBeat => {
                    info!("{} heartbeat", &hex_uid[.. 8]);
                    timeout.refresh();
                    client.refresh_timeout().expect("setting redis key expiration");
                },
                WorkerCommand::Hungup => {
                    debug!("{} hungup", &hex_uid[.. 8]);
                    hungup = true;
                },
                WorkerCommand::Print(job_id) => {
                    to_print.push_back(job_id);
                    info!("{} printing {}", &hex_uid[.. 8], job_id);
                    debug!("{:?}", to_print);
                },
            }
        }

        if let Some(_id) = printing {
            if accounting.update(counter(&state.ip).ok()) {
                if accounting.not_enough_credit() {
                    info!("{} not enought credit, aborting", &hex_uid[..]);
                    break
                }
                if accounting.finished() {
                    let (id, keep) = printing.take().unwrap();
                    if !keep {
                        delete_job_by_id(id, &connection).expect("deleting job from table");
                        info!("{} {} deleting job {}", &hex_uid[.. 8], task.user_id, id);
                    }
                    else {
                        info!("{} {} keeping job {}", &hex_uid[.. 8], task.user_id, id);
                    }
                }
                else {
                    timeout.refresh();
                }
            }
        }
        else if let Some(job_id) = to_print.pop_front() {
            if let Some(job_row) =
                select_full_job_of_user(task.user_id, job_id, &connection).expect("selecting job")
            {
                let mut job = Job::from((
                    job_row.id,
                    job_row.info.clone(),
                    job_row.options.clone(),
                    job_row.created,
                ));

                let counter = match counter(&state.ip) {
                    Ok(counter) => counter,
                    Err(_) => {
                        to_print.push_front(job_id);
                        break;
                    },
                };

                accounting.start(job.clone(), counter);

                if accounting.not_enough_credit() {
                    break;
                }

                let mut data = job_row.pdf;
                // preprocess pagerange
                if job.options.range != "" {
                    let path = &TmpFile::create(&data[..]).expect("creating tmp file");

                    trim_pages(path, &job.options.range).expect("trimming pages");

                    data = TmpFile::remove(path).expect("removing tmp file");
                }

                let buf: Vec<u8> = job.translate_for_printer(&task.uid[..], task.user_id, data);
                match LprConnection::new(
                    &state.ip, 20000, // socket timeout in ms
                ) {
                    Ok(mut connection) => {
                        match connection.print(&buf) {
                            Ok(_) => {
                                printing = Some((job.id, job.options.keep));
                                timeout.refresh();
                            },
                            Err(e) => {
                                error!("lpr: {:?}", e);
                                break
                            },
                        }
                    },
                    Err(e) => {
                        error!("lpr: {:?}", e);
                        break
                    },
                }
            }
        }
        else if hungup {
            info!("{} hungup", &hex_uid[.. 8]);
            break
        }

        if timeout.check() {
            info!("{} timeout", &hex_uid[.. 8]);
            break
        }
    }

    // clear jobqueue on every outcome in case printer wants to print more than
    // expected
    for _ in 0 .. 4 {
        if let Err(e) = clear(&state.ip) {
            error!("clearing jobqueue: {:?}", e);
        }
    }

    thread::sleep(time::Duration::from_millis(3000));

    for _ in 0 .. 4 {
        accounting.update(counter(&state.ip).ok());
    }

    info!("{} {} finished", &hex_uid[.. 8], task.user_id);
}
