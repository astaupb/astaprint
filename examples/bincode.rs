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
extern crate bincode;
use bincode::serialize;

extern crate model;
use model::task::dispatcher::DispatcherTask;

fn main()
{
    let task = DispatcherTask {
        user_id: 7,
        filename: "hello".to_string(),
        keep: Some(true),
        color: Some(true),
        a3: Some(true),
        duplex: Some(2),
        uid: vec![1, 2, 3],
    };
    println!("{:?}", serialize(&task));

    let task = DispatcherTask {
        user_id: 7,
        filename: "hello".to_string(),
        keep: None,
        color: None,
        a3: None,
        duplex: None,
        uid: vec![1, 2, 3],
    };
    println!("{:?}", serialize(&task));

    let task = DispatcherTask {
        user_id: 7,
        filename: "hello".to_string(),
        keep: Some(true),
        color: Some(false),
        a3: None,
        duplex: None,
        uid: vec![1, 2, 3],
    };
    println!("{:?}", serialize(&task));

    let task = DispatcherTask {
        user_id: 7,
        filename: "hello".to_string(),
        keep: Some(false),
        color: None,
        a3: None,
        duplex: None,
        uid: vec![1, 2, 3],
    };
    println!("{:?}", serialize(&task));
}
