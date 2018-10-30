/// AStAPrint-Backend - Crypto
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

use std::{
    fs::File,
    io::Read,
};

pub fn urandom(buf: &mut [u8])
{
    let mut file = File::open("/dev/urandom").expect("opening /dev/urandom");

    file.read_exact(buf).expect("reading /dev/urandom to buffer");
}

pub const SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE: c_ulonglong = 2;

pub const SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE: usize = 67_108_864;

pub const SODIUM_CRYPTO_PWHASH_ALGORITHM: c_int = 2;

#[link(name = "sodium")]

extern "C" {

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

// int crypto_pwhash(unsigned char * const out,
// unsigned long long outlen,
// const char * const passwd,
// unsigned long long passwdlen,
// const unsigned char * const salt,
// unsigned long long opslimit,
// size_t memlimit, int alg);}

pub fn verify_password(password: &str, hash: &[u8], salt: &[u8]) -> bool
{
    let out: [u8; 64] = [0; 64];

    unsafe {
        crypto_pwhash(
            out.as_ptr() as *const c_uchar,
            out.len() as c_ulonglong,
            password.as_ptr() as *const c_char,
            password.len() as c_ulonglong,
            salt.as_ptr() as *const c_uchar,
            SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE,
            SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE,
            SODIUM_CRYPTO_PWHASH_ALGORITHM,
        );
    };

    out[..] == hash[..]
}

pub fn hash_password(password: &str) -> (Vec<u8>, Vec<u8>)
{
    let out: [u8; 64] = [0; 64];

    let mut salt: [u8; 16] = [0; 16];

    urandom(&mut salt[..]);

    let _result = unsafe {
        crypto_pwhash(
            out.as_ptr() as *const c_uchar,
            out.len() as c_ulonglong,
            password.as_ptr() as *const c_char,
            password.len() as c_ulonglong,
            salt.as_ptr() as *const c_uchar,
            SODIUM_CRYPTO_PWHASH_OPSLIMIT_INTERACTIVE,
            SODIUM_CRYPTO_PWHASH_MEMLIMIT_INTERACTIVE,
            SODIUM_CRYPTO_PWHASH_ALGORITHM,
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
