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
use std::{
    thread,
    time,
};

use diesel::{
    prelude::*,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use r2d2_redis::RedisConnectionManager;

use lpr::LprConnection;

use jobs::{
    options::JobOptions,
    Job,
};

use printers::{
    accounting::Accounting,
};
use snmp::{
    session::SnmpSession,
    PrinterInterface,
};
use mysql::jobs::{
    Job as JobRow,
    select::*,
    delete::*,
};
use astacrypto::random_bytes;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkerTask
{
    pub job_id: u32,
    pub user_id: u32,
    pub uid: Vec<u8>,
    pub options: JobOptions,
}

impl WorkerTask
{
    pub fn new(id: u32, user_id: u32) -> WorkerTask
    {
        let uid = random_bytes(20);
        let options = JobOptions::default();
        WorkerTask {
            job_id: id,
            user_id,
            uid,
            options,
        }
    }
}

#[derive(Clone)]
pub struct WorkerState
{
    pub printer_interface: PrinterInterface,
    pub mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
    pub redis_pool: Pool<RedisConnectionManager>,
}

#[derive(PartialEq, Debug)]
pub enum WorkerCommand
{
    Print,
    Cancel,
}

pub fn work(task: WorkerTask, state: WorkerState)
{
    let hex_uid = hex::encode(&task.uid[..]);
    info!("{} print thread spawned for {}", hex_uid, task.user_id);

    let connection = state.mysql_pool.get().expect("getting connection from mysql pool");

    let job_row: JobRow = select_job(task.job_id, &connection)
        .expect("selecting job from database");

    let mut job = Job::from((job_row.id, job_row.info.clone(), job_row.options.clone(), job_row.created));

    let mut buf: Vec<u8> = job.translate_for_printer(&task.uid[..], task.user_id, job_row.pdf);

    let mut snmp_session = SnmpSession::new(state.printer_interface.clone());

    let mut accounting = Accounting::new(task.user_id, state.mysql_pool, state.redis_pool);

    if accounting.not_enough_credit() {
        info!("not enough credit for one page, aborting");
        return;
    }

    let counter_base = snmp_session
        .get_counter()
        .expect("reading base counter value");

    debug!("counter_base: {:?}", counter_base);

    let mut lpr_connection =
        LprConnection::new(&state.printer_interface.ip, 20000 /* socket timeout in ms */);
    lpr_connection.print(&mut buf).expect("printing job with lpr");

    let print_count = job.pages_to_print();

    // check energy status before initial waiting
    // 1 == ready
    let energy_stat = snmp_session.get_energy_stat()
        .expect("getting energy status of device");
    debug!("energy stat: {}", &energy_stat);
    thread::sleep(time::Duration::from_millis(match energy_stat {
        1 => 2000,
        _ => {
            match snmp_session.wake()
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
            .get_counter()
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

        if (current.total - counter_base.total) == print_count as i64 {
            debug!("current: {:?}", current);
            break true;
        }

        if accounting.not_enough_credit() {
            debug!("current: {:?}", current);
            info!("{} {} no credit left, clearing jobqueue", hex_uid, task.user_id);
            break false;
        }

        if loop_count > 800 {
            debug!("current: {:?}", current);
            warn!("{} {} timeout", hex_uid, task.user_id);
            break false;
        }
    };

    // clear jobqueue on every outcome in case printer wants to print more than expected
    snmp_session.clear_queue()
        .expect("clearing jobqueue");

    thread::sleep(time::Duration::from_millis(80));

    let current = snmp_session
        .get_counter()
        .expect("getting counter values");

    accounting.set_value(&(current.clone() - counter_base.clone()));
    accounting.finish();

    debug!("{} keep: {} - completed: {}", hex_uid, job.options.keep, completed);
    if !job.options.keep && completed {
        delete_job_by_id(job.id, task.user_id, &connection)
            .expect("deleting job from table");
    }

    info!("{} finished", hex_uid);
}
