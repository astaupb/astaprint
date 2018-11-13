/// AStACrypto - ffi
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
    c_void,
};

extern "C" {
    pub fn sodium_init() -> c_int;
    pub fn randombytes_buf(buf: *const c_void, len: usize);
}

#[repr(C)]
#[derive(Copy, Clone)]
#[repr(align(64))]
pub struct crypto_generichash_state
{
    pub h: [u64; 8usize],
    pub t: [u64; 2usize],
    pub f: [u64; 2usize],
    pub buf: [u8; 256usize],
    pub buflen: usize,
    pub last_node: u8,
    pub padding: [u8; 23usize],
}

#[link(name = "sodium")]
extern "C" {
    pub fn crypto_generichash_bytes_min() -> usize;
    pub fn crypto_generichash_bytes_max() -> usize;
    pub fn crypto_generichash_keybytes_min() -> usize;
    pub fn crypto_generichash_keybytes_max() -> usize;
    pub fn crypto_generichash(
        out: *const c_uchar,
        outlen: usize,
        _in: *const c_uchar,
        inlen: c_ulonglong,
        key: *const c_uchar,
        keylen: usize,
    ) -> c_int;
    pub fn crypto_generichash_init(
        state: *mut crypto_generichash_state,
        key: *const c_uchar,
        keylen: usize,
        outlen: usize,
    ) -> c_int;
    pub fn crypto_generichash_update(
        state: *mut crypto_generichash_state,
        in_: *const c_uchar,
        inlen: c_ulonglong,
    ) -> c_int;
    pub fn crypto_generichash_final(
        state: *mut crypto_generichash_state,
        out: *mut c_uchar,
        outlen: usize,
    ) -> c_int;
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
