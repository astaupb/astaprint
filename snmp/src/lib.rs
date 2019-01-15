/// AStAPrint-Worker
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

pub mod session;

pub mod counter;
pub mod control;
pub mod info;
pub mod status;

use std::str::FromStr;

use diesel::{
    prelude::*,
};

use mysql::printers::{
    select::*,
};

use crate::{
    counter::CounterOids,
    control::ControlOids,
    info::InfoOids,
    status::StatusOids,
};

#[derive(Debug, Clone)]
pub struct PrinterInterface
{
    pub ip: String,
    pub community: String,
    pub counter: CounterOids,
    pub control: ControlOids,
    pub status: StatusOids,
    pub info: InfoOids,
}

impl PrinterInterface
{
    pub fn from_device_id(
        device_id: u32,
        connection: &MysqlConnection,
    ) -> PrinterInterface
    {
        let (ip, community): (
            String,
            String,
        ) = select_ip_and_community_by_device_id(device_id, connection)
            .expect("selecting ip and community").unwrap();

        let (counter, control, info, status) = select_printer_objects_by_device_id(device_id, connection)
            .expect("selecting printer objects").unwrap();

        PrinterInterface {
            ip,
            community,
            counter: CounterOids::from(&counter),
            control: ControlOids::from(&control),
            info: InfoOids::from(&info),
            status: StatusOids::from(&status),
        }
    }

    pub fn counter_oids(&self) -> Vec<&Vec<u64>>
    {
        let mut counter_oids: Vec<&Vec<u64>> = Vec::with_capacity(5); 
        
        counter_oids.push(&self.counter.total);
        counter_oids.push(&self.counter.copy_total);
        counter_oids.push(&self.counter.copy_bw);
        counter_oids.push(&self.counter.print_total);
        counter_oids.push(&self.counter.print_bw);

        counter_oids

    }
}

fn parse_oid(oid: &str) -> Vec<u64>
{
    oid.split('.')
        .map(|x| u64::from_str(x).expect("converting oid str to u64"))
        .collect()
}

#[cfg(test)]
pub mod tests
{
    use mysql::create_mysql_pool;
    use mysql::printers::{
        select::{
            select_device_ids,
        },
    };
    use std::env;
    use crate::{
        session::SnmpSession,
        PrinterInterface,
    };
    fn dump_printer_objects(interface: PrinterInterface)
    {
        let mut snmp = SnmpSession::new(interface); 
        let counter = snmp.get_counter();
        println!("{:?}", counter);
        /*
        let status = snmp.get_status();
        println!("{:?}", status);
        let info = snmp.get_info();
        println!("{:?}", info);
        */
    }
    #[test]
    pub fn all_printers()
    {
        let url = format!("{}_test", env::var("ASTAPRINT_DATABASE_URL").unwrap());
        let pool = create_mysql_pool(&url, 2);
        let id_s = select_device_ids(&pool.get().unwrap()).unwrap();
        for id in id_s {
            let interface = PrinterInterface::from_device_id(id, &pool.get().unwrap());
            print!("{}: ", interface.ip);
            let mut snmp = SnmpSession::new(interface);
            println!("{:?}", snmp.get_counter());
        }
    }
}
