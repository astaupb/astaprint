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
extern crate legacy;
use legacy::tds::get_credit_optional;

extern crate mysql;
use mysql::{
    create_mysql_pool,
    import_credit,
    user::select::{
        select_user_credit_by_id,
        select_user_id,
    },
};

use std::env;

fn main()
{
    let mysql_url = env::var("ASTAPRINT_DATABASE_URL")
        .expect("reading ASTAPRINT_DATABASE_URL from environment");

    let connection = create_mysql_pool(&mysql_url, 1).get().unwrap();

    let user_ids = select_user_id(&connection).expect("selecting user ids");
    for id in user_ids.clone().iter().rev() {
        let credit = get_credit_optional(*id).unwrap_or(0);
        import_credit(*id, credit, &connection).expect("importing credit");
        if id % 2500 == 0 {
            println!("{} reached", id);
        }
    }
    // verify
    for id in user_ids {
        let credit = select_user_credit_by_id(id, &connection).expect("selection new credit");
        let legacy_credit = get_credit_optional(id).unwrap_or(0);
        assert_eq!(credit, legacy_credit);
        if id % 2500 == 0 {
            println!("{} reached", id);
        }
    }
}
