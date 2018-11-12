/// AStAPrint - Crypto - pwhash
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
use std::os::raw::{
    c_char,
    c_int,
    c_uchar,
    c_ulonglong,
};

use super::urandom;

const OPSLIMIT_INTERACTIVE: c_ulonglong = 2;
const MEMLIMIT_INTERACTIVE: usize = 67_108_864;
const ALGORITHM: c_int = 2; // argon2id
const SALTBYTES: usize = 16;
const HASHBYTES: usize = 64;

#[link(name = "sodium")]
extern "C" {
    pub fn crypto_pwhash_opslimit_interactive() -> usize;
    pub fn crypto_pwhash_memlimit_interactive() -> usize;
    pub fn crypto_pwhash_alg_default() -> c_int;
    pub fn crypto_pwhash_saltbytes() -> usize;
    pub fn crypto_pwhash_bytes_max() -> c_int;
    pub fn crypto_pwhash_bytes_min() -> c_int;
    pub fn crypto_pwhash(
        out: *const c_uchar,
        outlen: c_ulonglong,
        passwd: *const c_char,
        passwdlen: c_ulonglong,
        salt: *const c_uchar,
        opslimit: c_ulonglong,
        memlimit: usize,
        alg: c_int,
    ) -> c_int;
}

pub fn verify_password(password: &str, hash: &[u8], salt: &[u8]) -> bool
{
    let out: [u8; HASHBYTES] = [0; HASHBYTES];

    unsafe {
        crypto_pwhash(
            out.as_ptr() as *const c_uchar,
            out.len() as c_ulonglong,
            password.as_ptr() as *const c_char,
            password.len() as c_ulonglong,
            salt.as_ptr() as *const c_uchar,
            OPSLIMIT_INTERACTIVE,
            MEMLIMIT_INTERACTIVE,
            ALGORITHM,
        );
    };

    out[..] == hash[..]
}

pub fn hash_password(password: &str) -> (Vec<u8>, Vec<u8>)
{
    let out: [u8; HASHBYTES] = [0; HASHBYTES];

    let mut salt: [u8; SALTBYTES] = [0; SALTBYTES];

    urandom(&mut salt[..]);

    let _result = unsafe {
        crypto_pwhash(
            out.as_ptr() as *const c_uchar,
            out.len() as c_ulonglong,
            password.as_ptr() as *const c_char,
            password.len() as c_ulonglong,
            salt.as_ptr() as *const c_uchar,
            OPSLIMIT_INTERACTIVE,
            MEMLIMIT_INTERACTIVE,
            ALGORITHM,
        )
    };

    (out[..].to_vec(), salt[..].to_vec())
}

#[test]
fn verify()
{
    let (hash, salt) = hash_password("0123456789abcdef");

    assert!(verify_password("0123456789abcdef", &hash[..], &salt[..]));
}

#[test]
fn check_defaults()
{
    assert_eq!(OPSLIMIT_INTERACTIVE, unsafe { crypto_pwhash_opslimit_interactive() } as c_ulonglong);
    assert_eq!(MEMLIMIT_INTERACTIVE, unsafe { crypto_pwhash_memlimit_interactive() });
    assert_eq!(ALGORITHM, unsafe { crypto_pwhash_alg_default() });
    assert_eq!(SALTBYTES, unsafe { crypto_pwhash_saltbytes() });
    assert!(
        HASHBYTES < unsafe { crypto_pwhash_bytes_max() } as usize
            && HASHBYTES > unsafe { crypto_pwhash_bytes_min() } as usize
    );
}
