extern crate netsnmp_sys;

use self::netsnmp_sys::*;

use crate::{
    PrinterInterface,
    counter::CounterValues,
    status::StatusValues,
    info::InfoValues,
};

use std::{
    ffi::{
        CString, 
        CStr,
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
    pub fn get_string_bulk(&self, oids: Vec<&Vec<u64>>) -> Result<Vec<CString>, SnmpErr>
    {
        let pdu: *mut Struct_snmp_pdu;

        let mut response: *mut Struct_snmp_pdu;

        let mut response_values: Vec<&CStr> = Vec::with_capacity(oids.len());

        let bulk_len = oids.len();

        unsafe {
            response = mem::uninitialized();

            pdu = snmp_pdu_create(SNMP_MSG_GETBULK);

            (*pdu).errstat = bulk_len as i64; // errstat is non_repeaters in GETBULK
            (*pdu).errindex = bulk_len as i64; // errindex in max_repetitions in GETBULK

            for oid in oids {
                snmp_add_null_var(pdu, &oid[0], oid.len());
            }

            if snmp_synch_response(self.ptr, pdu, &mut response) == STAT_SUCCESS
            {
                if (*response).errstat == SNMP_ERR_NOERROR {
                    while !(*response).variables.is_null() {
                        {
                            let mut buf: [i8; 42] = [0; 42];
                            snprint_asciistring(
                                buf.as_mut_ptr(),
                                42,
                                *(*(*response).variables).val.string(),
                                (*(*response).variables).val_len, 
                            );
                            response_values.push(
                                CStr::from_ptr(buf.as_mut_ptr())
                            );
                        }
                        (*response).variables =
                            (*(*response).variables).next_variable;
                    }
                    if response_values.len() == bulk_len {
                        Ok(response_values.iter()
                            .map(|x|
                                CString::from(*x)
                            )
                            .collect()
                        )
                    } else {
                        Err(SnmpErr::BulkErr) 
                    }
                } else {
                    Err(SnmpErr::ResponseErr)
                }
            } else {
                Err(SnmpErr::SessionErr)
            }
        }
    }

    pub fn get_integer_bulk(&self, oids: Vec<&Vec<u64>>) -> Result<Vec<i64>, SnmpErr>
    {
        let pdu: *mut Struct_snmp_pdu;

        let mut response: *mut Struct_snmp_pdu;

        let mut response_values: Vec<i64> = Vec::with_capacity(oids.len());

        let bulk_len = oids.len();

        unsafe {
            response = mem::uninitialized();

            pdu = snmp_pdu_create(SNMP_MSG_GETBULK);

            (*pdu).errstat = bulk_len as i64; // errstat is non_repeaters in GETBULK
            (*pdu).errindex = bulk_len as i64; // errindex in max_repetitions in GETBULK

            for oid in oids {
                snmp_add_null_var(pdu, &oid[0], oid.len());
            }

            if snmp_synch_response(self.ptr, pdu, &mut response) == STAT_SUCCESS
            {
                if (*response).errstat == SNMP_ERR_NOERROR {
                    while !(*response).variables.is_null() {
                        if (*(*response).variables)._type == ASN_INTEGER {
                            println!("{:?}", (*(*response).variables)._type);
                            response_values.push(
                                **(*(*response).variables).val.integer() as i64
                            );
                        } else if (*(*response).variables)._type == ASN_OCTET_STR {
                            response_values.push(
                                **(*(*response).variables).val.integer() as i64
                            );
                        } else {
                            println!("unknown type: {}", (*(*response).variables)._type);
                            return Err(SnmpErr::TypeErr);
                        }
                        (*response).variables =
                            (*(*response).variables).next_variable;
                    }
                    if response_values.len() == bulk_len {
                        Ok(response_values)
                    } else {
                        Err(SnmpErr::BulkErr) 
                    }
                } else {
                    Err(SnmpErr::ResponseErr)
                }
            } else {
                Err(SnmpErr::SessionErr)
            }
        }
    }
    pub fn get_counter(&self) -> Result<CounterValues, SnmpErr>
    {
        Ok(CounterValues::from(
            self.get_integer_bulk(self.interface.counter.to_oid_vec())?
        ))
    }

    pub fn get_status(&self) -> Result<StatusValues, SnmpErr>
    {
        Ok(StatusValues::from(
            self.get_integer_bulk(self.interface.status.to_oid_vec())?
        ))
    }

    pub fn get_info(&self) -> Result<InfoValues, SnmpErr>
    {
        Ok(InfoValues::from(
            self.get_string_bulk(self.interface.info.to_oid_vec())? 
        )) 
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
