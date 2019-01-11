/// AStAPrint - Crypto
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
use std::os::raw::c_void;

pub mod ffi;
pub mod generichash;
pub mod pwhash;

pub use self::generichash::GenericHash;
pub use self::pwhash::PasswordHash;


pub fn init()
{
    unsafe { ffi::sodium_init() };
}

pub fn random_bytes(len: usize) -> Vec<u8>
{
    let mut buf: Vec<u8> = vec![0; len];
    unsafe {
        ffi::randombytes_buf(buf.as_mut_ptr() as *const c_void, buf.len() as usize);
    }
    buf
}
