pub mod counter;
pub mod session;

use astaprint_database::printer::{
    representation::*,
    select_printer_interface_information
};
use self::counter::CounterOids;

#[derive(Debug, Clone)]
pub struct PrinterInterface {
    pub ip: String,
    pub community: String,
    pub counter: CounterOids,
    pub queue_ctl: QueueControl,
    pub energy_ctl: EnergyControl,
}

impl PrinterInterface {
    pub fn from_device_id(device_id: &u16) -> PrinterInterface {
        let (row, queue_ctl, energy_ctl, community, ip)
            :(Counter, QueueCtl, EnergyCtl, String, String) = select_printer_interface_information(device_id);
        PrinterInterface {
            ip: ip,
            community: community,
            counter: CounterOids {
                total: vec_from_oid_str(&row.total),
                print_black: vec_from_oid_str(&row.print_black),
                print_color: row.print_color.map(|s| vec_from_oid_str(&s)),
                copy_black: vec_from_oid_str(&row.copy_black),
                copy_color: row.copy_color.map(|s| vec_from_oid_str(&s)),
            },
            queue_ctl: QueueControl::from(queue_ctl),
            energy_ctl: EnergyControl::from(energy_ctl),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueueControl {
    pub oid: Vec<u64>,
    pub cancel: i32, 
    pub clear: i32,
}
impl From<QueueCtl> for QueueControl {
    fn from(queue_ctl: QueueCtl) -> Self {
        QueueControl {
            oid: vec_from_oid_str(&queue_ctl.oid),
            cancel: queue_ctl.cancel,
            clear: queue_ctl.clear,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnergyControl {
    pub oid: Vec<u64>,
    pub wake: i32,
    pub sleep: i32,
}

impl From<EnergyCtl> for EnergyControl {
    fn from(energy_ctl: EnergyCtl) -> Self {
        EnergyControl {
            oid: vec_from_oid_str(&energy_ctl.oid),
            wake: energy_ctl.wake,
            sleep: energy_ctl.sleep,
        }
    }
}


fn vec_from_oid_str(oid: &str) -> Vec<u64> {
    use std::str::FromStr;
    oid.split(".")
        .map(|x| u64::from_str(x).expect("converting oid str to u64"))
        .collect()
}

#[cfg(test)]
mod snmp_tests {
    use astaprint_database::printer::select_device_ids;
    use super::PrinterInterface;
    use snmp::session::SnmpSession;
    #[test]
    fn get_counter_values() {
        for mut interface in select_device_ids()
            .iter()
            .map(|id| PrinterInterface::from_device_id(id))
        {
            let session = SnmpSession::new(&interface.ip, &interface.community);
            match session.get_counter_values(&mut interface.counter) {
                Ok(value) => {
                    println!("{}: {:#?}", &interface.ip, value);
                }
                Err(e) => {
                    println!("{}: {:?}", &interface.ip, e);
                }
            }
        }
    }
}
