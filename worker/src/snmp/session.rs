extern crate netsnmp_sys;

use self::netsnmp_sys::*;

use snmp::counter::{
    CounterOids,
    CounterValues,
};

use std::{
    ffi::CString,
    mem,
    os,
};

#[derive(Debug)]

pub enum SnmpErr
{
    TypeErr,
    SessionErr,
    ResponseErr,
}

pub struct SnmpSession
{
    ptr: *mut Struct_snmp_session,
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
    pub fn new(ip: &str, community: &str) -> SnmpSession
    {
        init(b"counter listener");

        let peer_name = CString::new(ip).unwrap().into_raw();

        let peer_community_len = community.len();

        let peer_community = CString::new(community).unwrap().into_raw();

        unsafe {
            let mut session: Struct_snmp_session = mem::uninitialized();

            snmp_sess_init(&mut session);

            session.peername = peer_name;

            session.version = SNMP_VERSION_1;

            session.community = peer_community as *mut u8;

            session.community_len = peer_community_len;

            SnmpSession {
                ptr: snmp_open(&mut session),
            }
        }
    }

    pub fn set_integer(&self, oid: &mut [u64], value: i32) -> Result<i64, SnmpErr>
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

            if snmp_synch_response(self.ptr, pdu, &mut response) == STAT_SUCCESS {
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

            if snmp_synch_response(self.ptr, pdu, &mut response) == STAT_SUCCESS {
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

    pub fn get_counter_values(&self, counter: &mut CounterOids) -> Result<CounterValues, SnmpErr>
    {
        let total = self.get_integer(&mut counter.total[..])?;

        let print_black = self.get_integer(&mut counter.print_black[..])?;

        let copy_black = self.get_integer(&mut counter.copy_black[..])?;

        let print_color = match counter.print_color {
            Some(ref mut oid) => Some(self.get_integer(oid)?),
            None => None,
        };

        let copy_color = match counter.copy_color {
            Some(ref mut oid) => Some(self.get_integer(oid)?),
            None => None,
        };

        Ok(CounterValues {
            total,
            print_black,
            copy_black,
            print_color,
            copy_color,
        })
    }
}
