/// AStACrypto - pwhash
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

use super::{
    ffi::*,
    init,
    random_bytes,
};

const OPSLIMIT_INTERACTIVE: c_ulonglong = 2;
const MEMLIMIT_INTERACTIVE: usize = 67_108_864;
const ALGORITHM: c_int = 2; // argon2id
const SALTBYTES: usize = 16;
const HASHBYTES: usize = 64;

pub struct PasswordHash;

impl PasswordHash
{
    pub fn create(input: &[u8]) -> (Vec<u8>, Vec<u8>)
    {
        init();
        let salt = random_bytes(SALTBYTES);
        let hash = vec![0; HASHBYTES];
        unsafe {
            crypto_pwhash(
                hash.as_ptr() as *const c_uchar,
                hash.len() as c_ulonglong,
                input.as_ptr() as *const c_char,
                input.len() as c_ulonglong,
                salt.as_ptr() as *const c_uchar,
                OPSLIMIT_INTERACTIVE,
                MEMLIMIT_INTERACTIVE,
                ALGORITHM,
            );
        }
        (hash, salt)
    }

    pub fn with_salt(input: &[u8], salt: &[u8]) -> Vec<u8>
    {
        init();
        assert_eq!(salt.len(), SALTBYTES);
        let hash = vec![0; HASHBYTES];
        unsafe {
            crypto_pwhash(
                hash.as_ptr() as *const c_uchar,
                hash.len() as c_ulonglong,
                input.as_ptr() as *const c_char,
                input.len() as c_ulonglong,
                salt.as_ptr() as *const c_uchar,
                OPSLIMIT_INTERACTIVE,
                MEMLIMIT_INTERACTIVE,
                ALGORITHM,
            );
        }
        hash
    }
}

#[cfg(test)]
mod tests
{
    use crate::{
        ffi::*,
        pwhash::*,
    };
    #[test]
    fn verify()
    {
        let input = random_bytes(108);
        let (hash, salt) = PasswordHash::create(&input[..]);

        assert!(PasswordHash::with_salt(&input[..], &salt[..]) == hash);
    }
    #[test]
    fn check_constants()
    {
        unsafe {
            assert_eq!(crypto_pwhash_opslimit_interactive(), OPSLIMIT_INTERACTIVE as usize);
            assert_eq!(crypto_pwhash_memlimit_interactive(), MEMLIMIT_INTERACTIVE);
            assert_eq!(crypto_pwhash_alg_default(), ALGORITHM);
            assert_eq!(crypto_pwhash_saltbytes(), SALTBYTES);

            assert!(crypto_pwhash_bytes_max() as usize >= HASHBYTES);
            assert!(crypto_pwhash_bytes_min() as usize <= HASHBYTES);
        }
    }
}
