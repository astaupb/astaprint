extern crate netsnmp_sys;

use self::netsnmp_sys::*;

use crate::{
    PrinterInterface,
    counter::CounterValues,
    status::StatusValues,
};

use std::{
    ffi::{
        CString, 
    },
    mem,
    os,
};

#[derive(Debug)]
pub enum SnmpErr
{
    TypeErr,
    SessionErr,
    ResponseErr,
    BulkErr,
}

pub struct SnmpSession
{
    ptr: *mut Struct_snmp_session,
    interface: PrinterInterface,
}

impl Drop for SnmpSession
{
    fn drop(&mut self)
    {
        unsafe {
            snmp_close(self.ptr);
        }
    }
}

impl SnmpSession
{
    pub fn new(interface: PrinterInterface) -> SnmpSession
    {
        init(b"snmpsession");

        let peer_name = CString::new(interface.ip.clone()).unwrap().into_raw();

        let peer_community_len = interface.community.len();

        let peer_community = CString::new(interface.community.clone()).unwrap().into_raw();

        unsafe {
            let mut session: Struct_snmp_session = mem::uninitialized();

            snmp_sess_init(&mut session);

            session.peername = peer_name;

            session.version = SNMP_VERSION_2c;

            session.community = peer_community as *mut u8;

            session.community_len = peer_community_len;

            SnmpSession {
                ptr: snmp_open(&mut session),
                interface,
            }
        }
    }

    pub fn set_integer(
        &self,
        oid: &mut [u64],
        value: i32,
    ) -> Result<i64, SnmpErr>
    {
        let pdu: *mut Struct_snmp_pdu;

        let mut response: *mut Struct_snmp_pdu;

        let value_ptr = &value as *const os::raw::c_int;

        unsafe {
            response = mem::uninitialized();

            pdu = snmp_pdu_create(SNMP_MSG_SET);

            snmp_pdu_add_variable(
                pdu,
                &mut oid[0],
                oid.len(),
                ASN_INTEGER,
                value_ptr as *const os::raw::c_void,
                mem::size_of::<os::raw::c_int>(),
            );

            if snmp_synch_response(self.ptr, pdu, &mut response) == STAT_SUCCESS
            {
                if (*response).errstat == SNMP_ERR_NOERROR {
                    if (*(*response).variables)._type == ASN_INTEGER {
                        Ok(**(*(*response).variables).val.integer())
                    } else {
                        Err(SnmpErr::TypeErr)
                    }
                } else {
                    Err(SnmpErr::ResponseErr)
                }
            } else {
                Err(SnmpErr::SessionErr)
            }
        }
    }

    pub fn get_integer(&self, oid: &mut [u64]) -> Result<u64, SnmpErr>
    {
        let pdu: *mut Struct_snmp_pdu;

        let mut response: *mut Struct_snmp_pdu;

        unsafe {
            response = mem::uninitialized();

            pdu = snmp_pdu_create(SNMP_MSG_GET);

            snmp_add_null_var(pdu, &mut oid[0], oid.len());

            if snmp_synch_response(self.ptr, pdu, &mut response) == STAT_SUCCESS
            {
                if (*response).errstat == SNMP_ERR_NOERROR {
                    if (*(*response).variables)._type == ASN_INTEGER {
                        Ok((**(*(*response).variables).val.integer()) as u64)
                    } else {
                        Err(SnmpErr::TypeErr)
                    }
                } else {
                    Err(SnmpErr::ResponseErr)
                }
            } else {
                Err(SnmpErr::SessionErr)
            }
        }
    }

    pub fn get_counter(&mut self) -> Result<CounterValues, SnmpErr>
    {
        Ok(CounterValues {
            total: self.get_integer(&mut self.interface.counter.total.clone()[..])? as i64,
            copy_total: self.get_integer(&mut self.interface.counter.copy_total.clone()[..])? as i64,
            copy_bw: self.get_integer(&mut self.interface.counter.copy_bw.clone()[..])? as i64,
            print_total: self.get_integer(&mut self.interface.counter.print_total.clone())? as i64,
            print_bw: self.get_integer(&mut self.interface.counter.print_bw.clone())? as i64,
        })
    }
    pub fn get_status(&self) -> Result<StatusValues, SnmpErr>
    {
        Ok(StatusValues {
            scan: self.get_integer(&mut self.interface.status.scan.clone()[..])? as i64,
            copy: self.get_integer(&mut self.interface.status.copy.clone()[..])? as i64,
            toner: self.get_integer(&mut self.interface.status.toner.clone()[..])? as i64,
            tray_1: self.get_integer(&mut self.interface.status.tray_1.clone()[..])? as i64,
            tray_2: self.get_integer(&mut self.interface.status.tray_2.clone()[..])? as i64,
            tray_3: self.get_integer(&mut self.interface.status.tray_3.clone()[..])? as i64,
            tray_4: self.get_integer(&mut self.interface.status.tray_4.clone()[..])? as i64,
        })
    }

    pub fn get_energy_stat(&self) -> Result<u64, SnmpErr>
    {
        self.get_integer(
            &mut self.interface.control.energy.clone()[..],
        )
    }

    pub fn wake(&mut self) -> Result<i64, SnmpErr>
    {
        self.set_integer(
            &mut self.interface.control.energy.clone()[..],
            self.interface.control.wake,
        )
    }

    pub fn wait_for_wake(&mut self) -> Result<(), SnmpErr>
    {
        use std::{thread, time};
        let mut count = 0;
        while (self.get_energy_stat()? != 1) && (count < 10) {
            self.wake()?; 
            count += 1;
            thread::sleep(time::Duration::from_secs(2));
        }
        Ok(())
    }

    pub fn sleep(&mut self) -> Result<i64, SnmpErr>
    {
        self.set_integer(
            &mut self.interface.control.energy.clone()[..],
            self.interface.control.sleep,
        )
    }

    pub fn wait_for_sleep(&mut self) -> Result<(), SnmpErr>
    {
        use std::{thread, time}; 
        let mut count = 0;
        while (self.get_energy_stat()? != 4) && (count < 10) {
            self.sleep()?; 
            count += 1;
            thread::sleep(time::Duration::from_secs(2));
        }
        Ok(())
    }

    pub fn clear_queue(&mut self) -> Result<i64, SnmpErr>
    {
        self.set_integer(
            &mut self.interface.control.queue.clone()[..],
            self.interface.control.clear,
        )
    }
}
