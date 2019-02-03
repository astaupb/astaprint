// AStAPrint
// Copyright (C) 2018, 2019 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//
// This file is part of AStAPrint
//
// AStAPrint is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use std::mem::transmute;

pub fn merge_x_api_key(
    user_id: u32,
    token: Vec<u8>,
) -> Result<Vec<u8>, ()>
{
    if token.len() != 128 {
        warn!("invalid token length: {}", token.len());
        return Err(())
    }
    let mut key = Vec::with_capacity(132);
    let user_id_buf: [u8; 4] = unsafe { transmute::<u32, [u8; 4]>(user_id) };
    key.extend_from_slice(&user_id_buf[..]);
    key.extend(token);
    Ok(key)
}

pub fn split_x_api_key(mut key: Vec<u8>) -> Result<(u32, Vec<u8>), ()>
{
    if key.len() != 132 {
        warn!("invalid key length: {}", key.len());
        return Err(())
    }
    let token = key.split_off(4);

    let mut user_id: [u8; 4] = [0; 4];
    user_id.copy_from_slice(&key[.. 4]);

    let user_id = unsafe { transmute::<[u8; 4], u32>(user_id) };

    Ok((user_id, token))
}

#[test]
fn transmuting()
{
    use sodium::random_bytes;
    let user_id: u32 = 420;
    let token = random_bytes(128);
    let merged = merge_x_api_key(user_id, token.clone()).unwrap();
    let (user_id_splitted, token_splitted) = split_x_api_key(merged).unwrap();
    assert_eq!(user_id, user_id_splitted);
    assert_eq!(token, token_splitted);
}
