use snmp::{
    session::SnmpSession,
    PrinterInterface,
};


use std::env;

use std::thread;

use mysql::{
    create_mysql_pool,
    printers::select::select_device_ids,
};

pub fn main()
{
    let url = format!("{}_test", env::var("ASTAPRINT_DATABASE_URL").unwrap());
    let pool = create_mysql_pool(&url, 2);
    let id_s = select_device_ids(&pool.get().unwrap()).unwrap();
    for id in id_s {
        let interface = PrinterInterface::from_device_id(id, &pool.get().unwrap());
        thread::spawn(move || {
            let mut snmp = SnmpSession::new(interface);
            snmp.wait_for_sleep()
        });
    }
}
