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
    thread,
    time,
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
    let hex_uid = hex::encode(&task.uid[..]);
    info!("{}#{}", &hex_uid[.. 8], task.user_id);
    let client = CommandClient::from((&client, &hex_uid[..]));

    let connection = state.mysql_pool.get().expect("getting connection from mysql pool");

    let counter_base = counter(state.device_id).expect("getting counter base");
    let mut current = counter_base.clone();

    let mut accounting =
        Accounting::new(task.user_id, counter_base.clone(), state.mysql_pool, state.redis_pool);

    if accounting.not_enough_credit() {
        info!("not enough credit for one page, aborting");
        return
    }

    debug!("counter_base: {:?}", counter_base);

    let _wake = wake(state.device_id);
    let mut timeout = TimeOut::new(60);
    let mut print_count = 0;
    let mut hungup = false;
    let mut last_value = counter_base.total;
    let mut print_jobs: Vec<Job> = Vec::new();

    let command_receiver = client.get_command_receiver();
    let completed = loop {
        match command_receiver.try_recv() {
            Ok(command) => {
                debug!("timeout: {:?}", timeout);
                debug!("command: {:?}", command);
                match command {
                    WorkerCommand::Cancel => break false,
                    WorkerCommand::HeartBeat => {
                        timeout.refresh();
                    },
                    WorkerCommand::Hungup => {
                        hungup = true;
                    },
                    WorkerCommand::Print(job_id) => {
                        if let Some(job_row) = select_full_job(job_id, &connection)
                            .expect("selecting job from database")
                        {
                            let mut job = Job::from((
                                job_row.id,
                                job_row.info.clone(),
                                job_row.options.clone(),
                                job_row.created,
                            ));

                            let color = job.options.color;

                            let buf: Vec<u8> = job.translate_for_printer(
                                &task.uid[..],
                                task.user_id,
                                if color {
                                    job_row.pdf
                                }
                                else {
                                    job_row.pdf_bw
                                },
                            );

                            let mut lpr_connection = LprConnection::new(
                                &state.ip, 20000, // socket timeout in ms
                            );

                            lpr_connection.print(&buf).expect("printing job with lpr");

                            print_count += job.pages_to_print();
                            print_jobs.push(job);
                        }
                        else {
                            info!(
                                "{}#{} unable to find job#{}",
                                &hex_uid[.. 8],
                                task.user_id,
                                job_id
                            );
                        }
                    },
                }
            },
            Err(_) => (),
        }
        if let Ok(counter) = counter(state.device_id) {
            current = counter;
        }

        if current.total > last_value {
            debug!("{:?}", current);
            timeout.refresh();
            last_value = current.total;
            accounting.set_value(current.clone() - counter_base.clone());
        }

        if hungup && !print_jobs.is_empty() {
            if (current.total - counter_base.total) == i64::from(print_count) {
                debug!("current: {:?}", current);
                break true
            }
        }

        if accounting.not_enough_credit() {
            info!("{}#{} not enough credit, aborting", &hex_uid[.. 8], task.user_id);
            break false
        }

        if timeout.check() {
            info!("{}#{} timeout", &hex_uid[.. 8], task.user_id);
            break false
        }
    };

    // clear jobqueue on every outcome in case printer wants to print more than
    // expected
    let _clear = clear(state.device_id);

    thread::sleep(time::Duration::from_millis(80));

    if let Ok(counter) = counter(state.device_id) {
        current = counter;
    };
    accounting.set_value(current.clone() - counter_base.clone());
    accounting.finish();

    debug!("completed: {:?}, print_jobs: {:?}", completed, print_jobs);

    if completed {
        for job in print_jobs {
            if !job.options.keep {
                delete_job_by_id(job.id, &connection).expect("deleting job from table");
            }
        }
    }

    info!("{}#{} finished", &hex_uid[.. 8], task.user_id);
}
