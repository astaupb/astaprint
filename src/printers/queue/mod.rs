/// AStAPrint
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
pub mod get;
pub mod post;

use model::{
    job::Job,
    task::worker::{
        WorkerState,
        WorkerTask,
        WorkerCommand,
    },
};

use std::{
    thread,
    time,
};

use lpr::LprConnection;

use printers::accounting::Accounting;

use mysql::jobs::{
    delete::*,
    select::*,
    Job as JobRow,
};

use redis::queue::{
    TaskQueueClient, CommandClient,
};

use snmp::session::SnmpSession;

pub fn work(task: WorkerTask, state: WorkerState, client: TaskQueueClient<WorkerTask, WorkerCommand>)
{
    let hex_uid = hex::encode(&task.uid[..]);
    info!("{} worker thread spawned for {}", hex_uid, task.user_id);
    let client = CommandClient::from((&client, &hex_uid[..]));

    let connection =
        state.mysql_pool.get().expect("getting connection from mysql pool");

    let mut snmp_session = SnmpSession::new(state.printer_interface.clone());

    let counter_base =
        snmp_session.get_counter().expect("reading base counter value");

    let mut accounting = Accounting::new(
        task.user_id,
        counter_base.clone(),
        state.mysql_pool,
        state.redis_pool,
    );

    if accounting.not_enough_credit() {
        info!("not enough credit for one page, aborting");
        return;
    }

    debug!("counter_base: {:?}", counter_base);

/*

*/
    // check energy status before initial waiting
    // 1 == ready
    let energy_stat =
        snmp_session.get_energy_stat().expect("getting energy status of device");
    debug!("energy stat: {}", &energy_stat);
    thread::sleep(time::Duration::from_millis(match energy_stat {
        1 => 2000,
        _ => {
            match snmp_session.wake() {
                Ok(_) => 10000,
                Err(_) => 12000,
            }
        },
    }));
    let mut loop_count = 0;
    let mut print_count = 0;
    let mut hungup = false;
    let mut last_value = counter_base.total;
    let mut print_jobs: Vec<Job> = Vec::new();

    let completed = loop {
        if let Some(command) = client.receive_command().expect("receiving command") {
            debug!("{:?}", command);
            match command {
                WorkerCommand::Cancel => {
                    break false;
                },
                WorkerCommand::Hungup => {
                    hungup = true;
                },
                WorkerCommand::Print(job_id) => {
                    let job_row: JobRow =
                        select_job(job_id, &connection).expect("selecting job from database");

                    let mut job = Job::from((
                        job_row.id,
                        job_row.info.clone(),
                        job_row.options.clone(),
                        job_row.created,
                    ));

                    let buf: Vec<u8> =
                        job.translate_for_printer(&task.uid[..], task.user_id, job_row.pdf);

                    let mut lpr_connection = LprConnection::new(
                        &state.printer_interface.ip,
                        20000, // socket timeout in ms
                    );

                    lpr_connection.print(&buf).expect("printing job with lpr");

                    print_count += job.pages_to_print();
                    print_jobs.push(job);
                },
            }
        }
        thread::sleep(time::Duration::from_millis(20));
        let current = snmp_session.get_counter().expect("getting counter values");

        // reset loop count if another page is printed
        if current.total > last_value {
            debug!("current: {:?}", current);
            last_value = current.total;
            loop_count = 0;
            accounting.set_value(current.clone() - counter_base.clone());
        } else {
            loop_count += 1;
            if loop_count % 200 == 0 {
                debug!("loop_count: {:?}", loop_count);
            }
        }

        if hungup && !print_jobs.is_empty() {
            if (current.total - counter_base.total) == i64::from(print_count) {
                debug!("current: {:?}", current);
                break true;
            }
        }

        if accounting.not_enough_credit() {
            debug!("current: {:?}", current);
            info!("{} {} no credit left, clearing jobqueue", hex_uid, task.user_id);
            break false;
        }

        if hungup && loop_count > 500 && !print_jobs.is_empty() {
            warn!("{} {} jobs timeout", hex_uid, task.user_id);
            break false;
        }
    };

    // clear jobqueue on every outcome in case printer wants to print more than
    // expected
    snmp_session.clear_queue().expect("clearing jobqueue");

    thread::sleep(time::Duration::from_millis(80));

    let current = snmp_session.get_counter().expect("getting counter values");

    accounting.set_value(current.clone() - counter_base.clone());
    accounting.finish();

    debug!("{} completed: {}, print_jobs: {:?}", hex_uid, completed, print_jobs);

    if completed {
        for job in print_jobs {
            if !job.options.keep {
                delete_job_by_id(job.id, &connection)
                    .expect("deleting job from table");
            }
        }
    }

    info!("{} finished", hex_uid);
}
