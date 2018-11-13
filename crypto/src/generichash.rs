/// AStACrypto - generichash
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
    c_uchar,
    c_ulonglong,
};

use super::{
    ffi::*,
    init,
    random_bytes,
};

const HASHBYTES: usize = 64;
const SALTBYTES: usize = 64;

pub struct GenericHash
{
    state: crypto_generichash_state,
}

impl GenericHash
{
    pub fn create(input: &[u8]) -> (Vec<u8>, Vec<u8>)
    {
        init();
        let key = random_bytes(SALTBYTES);
        let mut hash = GenericHash::init(&key[..]);
        for chunk in input.chunks(HASHBYTES) {
            hash.process(chunk);
        }
        let hash = hash.finish();
        (hash.to_vec(), key)
    }

    pub fn with_salt(input: &[u8], salt: &[u8]) -> Vec<u8>
    {
        init();
        let mut hash = GenericHash::init(salt);
        for chunk in input.chunks(HASHBYTES) {
            hash.process(chunk);
        }
        hash.finish().to_vec()
    }

    fn init(salt: &[u8]) -> GenericHash
    {
        assert!(salt.len() == SALTBYTES);
        let mut state = crypto_generichash_state {
            h: [0; 8],
            t: [0; 2],
            f: [0; 2],
            buf: [0; 256],
            buflen: 0,
            last_node: 0,
            padding: [0; 23],
        };
        unsafe {
            crypto_generichash_init(
                &mut state as *mut crypto_generichash_state,
                salt.as_ptr() as *const c_uchar,
                SALTBYTES,
                HASHBYTES,
            );
        }
        GenericHash {
            state,
        }
    }

    fn process(&mut self, chunk: &[u8])
    {
        unsafe {
            crypto_generichash_update(
                &mut self.state as *mut crypto_generichash_state,
                chunk.as_ptr() as *const c_uchar,
                chunk.len() as c_ulonglong,
            );
        }
    }

    fn finish(mut self) -> [u8; HASHBYTES]
    {
        let mut out: [u8; HASHBYTES] = [0; HASHBYTES];
        unsafe {
            crypto_generichash_final(
                &mut self.state as *mut crypto_generichash_state,
                out.as_mut_ptr() as *mut c_uchar,
                out.len() as usize,
            );
        }
        out
    }
}

#[cfg(test)]
mod tests
{
    use crate::{
        ffi::*,
        generichash::{
            GenericHash,
            HASHBYTES,
            SALTBYTES,
        },
        random_bytes,
    };
    #[test]
    pub fn verify()
    {
        let input = random_bytes(108);
        let (hash, salt) = GenericHash::create(&input[..]);

        assert!(GenericHash::with_salt(&input[..], &salt[..]) == hash);
    }
    #[test]
    fn check_constants()
    {
        let hashbytes_min = unsafe { crypto_generichash_bytes_min() };
        let hashbytes_max = unsafe { crypto_generichash_bytes_max() };

        assert!(HASHBYTES <= hashbytes_max);
        assert!(HASHBYTES >= hashbytes_min);

        let keybytes_min = unsafe { crypto_generichash_keybytes_min() };
        let keybytes_max = unsafe { crypto_generichash_keybytes_max() };

        assert!(SALTBYTES <= keybytes_max);
        assert!(SALTBYTES >= keybytes_min);
    }
}
