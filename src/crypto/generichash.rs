/// AStAPrint - Crypto - generichash
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
    c_int,
    c_uchar,
    c_ulonglong,
};

use super::urandom;

const KEY_BYTES: usize = 64;
const HASH_BYTES: usize = 64;

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
}

pub fn generichash(input: &[u8], key: &[u8]) -> Vec<u8>
{
    let hash: Vec<u8> = Vec::with_capacity(HASH_BYTES);

    unsafe {
        crypto_generichash(
            hash.as_ptr() as *const c_uchar,
            hash.len() as usize,
            input.as_ptr() as *const c_uchar,
            input.len() as c_ulonglong,
            key.as_ptr() as *const c_uchar,
            key.len() as usize,
        );
    }

    hash
}

pub fn create(input: &[u8]) -> (Vec<u8>, Vec<u8>)
{
    let mut key: Vec<u8> = Vec::with_capacity(KEY_BYTES);
    urandom(&mut key);

    let hash: Vec<u8> = Vec::with_capacity(HASH_BYTES);

    generichash(input, &key[..]);

    (hash.to_vec(), key.to_vec())
}

pub fn verify(input: &[u8], output: &[u8], key: &[u8]) -> bool
{
    generichash(input, key)[..] == output[..]
}

#[test]
fn verify_generichash()
{
    let mut input: Vec<u8> = Vec::with_capacity(108);
    urandom(&mut input[..]);

    let (hash, key) = create(&input[..]);

    assert!(verify(&input[..], &hash[..], &key[..]));
}

#[test]
fn check_parameter()
{
    let hashbytes_min = unsafe { crypto_generichash_bytes_min() };
    let hashbytes_max = unsafe { crypto_generichash_bytes_max() };

    assert!(HASH_BYTES <= hashbytes_max && HASH_BYTES >= hashbytes_min);

    let keybytes_min = unsafe { crypto_generichash_keybytes_min() };
    let keybytes_max = unsafe { crypto_generichash_keybytes_max() };

    assert!(KEY_BYTES <= keybytes_max && HASH_BYTES >= keybytes_min);
}
