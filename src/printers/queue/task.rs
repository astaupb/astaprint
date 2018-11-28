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
    r2d2::{
        Pool, ConnectionManager,
    },
    delete,
    prelude::*,
};

use lpr::LprConnection;

use jobs::{
    options::JobOptions,
    uid::UID,
    *,
};

use printers::{
    accounting::Accounting,
    snmp::{
        session::SnmpSession,
        PrinterInterface,
    },
};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerTask
{
    pub job_id: u32,
    pub user_id: u32,
    pub uid: Vec<u8>,
    pub options: JobOptions,
}

#[derive(Clone)]
pub struct WorkerState
{
    pub printer_interface: PrinterInterface,
    pub mysql_pool: Pool<ConnectionManager<MysqlConnection>>,
}

#[derive(PartialEq, Debug)]
pub enum WorkerCommand
{
    Print,
    Cancel,
}

pub fn work(
    task: WorkerTask,
    mut state: WorkerState,
)
{
    let uid = UID::from(task.uid.clone());
    info!("{:x} print thread spawned for {}", uid, task.user_id);

    let connection = state.mysql_pool.get()
        .expect("getting connection from mysql pool");

    let mut job: Job = jobs::table
        .select(jobs::all_columns)
        .filter(jobs::id.eq(task.job_id))
        .filter(jobs::user_id.eq(task.user_id))
        .first(&connection)
        .expect("reading job from database");

    let mut buf: Vec<u8> = job.translate_for_printer(&uid);
    let info = job.info();
    let options = job.options();

    let snmp_session = SnmpSession::new(&state.printer_interface.ip, &state.printer_interface.community);

    let mut accounting = Accounting::new(task.user_id, info.color, &state.mysql_pool);

    if accounting.not_enough_credit() {
        info!("not enough credit for one page, aborting");
        return;
    }

    let counter_base = snmp_session
        .get_counter_values(&mut state.printer_interface.counter)
        .expect("reading base counter value");

    debug!("counter_base: {:?}", counter_base);

    let mut lpr_connection =
        LprConnection::new(&state.printer_interface.ip, 20000 /* socket timeout in ms */);
    lpr_connection.print(&mut buf).expect("printing job with lpr");

    let print_count = job.pages_to_print();

    // check energy status before initial waiting
    // 1 == ready
    let energy_stat = snmp_session
        .get_integer(&mut state.printer_interface.energy_ctl.oid[..])
        .expect("getting energy status of device");
    debug!("energy stat: {}", &energy_stat);
    thread::sleep(time::Duration::from_millis(match energy_stat {
        1 => 2000,
        _ => {
            match snmp_session
                .set_integer(&mut state.printer_interface.energy_ctl.oid[..], state.printer_interface.energy_ctl.wake)
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
            .get_counter_values(&mut state.printer_interface.counter)
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
            info!("{:x} {} no credit left, clearing jobqueue", uid, task.user_id);
            break false;
        }

        if loop_count > 420 {
            debug!("current: {:?}", current);
            warn!("{:x} {} timeout", uid, task.user_id);
            break false;
        }

    };

    // clear jobqueue on every outcome in case printer wants to print more than expected
    snmp_session
        .set_integer(&mut state.printer_interface.queue_ctl.oid[..], state.printer_interface.queue_ctl.clear)
        .expect("clearing jobqueue");

    accounting.finish();

    debug!("{:x} keep: {} - completed: {}", uid, options.keep, completed);
    if !options.keep && completed {
        delete(jobs::table.filter(jobs::id.eq(task.job_id)).filter(jobs::user_id.eq(task.user_id)))
            .execute(&connection)
            .expect("deleting job from table");
    }

    info!("{:x} finished", uid);
}
