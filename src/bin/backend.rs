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
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate model;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate base64;

extern crate logger;

extern crate astaprint;
extern crate mysql;
extern crate redis;

use std::collections::HashMap;

use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
};

use redis::{
    get_redis_pool,
    queue::TaskQueueClient,
    share::Share,
    store::Store,
    Redis,
};

use mysql::{
    get_mysql_pool,
    printers::select::select_device_ids,
};

use model::task::{
    dispatcher::DispatcherTask,
    worker::{
        WorkerCommand,
        WorkerTask,
    },
};

use logger::Logger;

use astaprint::{
    admin::{
        get::*,
        post::*,
        put::*,
        tokens::*,
    },
    jobs::{
        delete::*,
        get::*,
        info::{
            get::*,
            put::*,
        },
        options::{
            get::*,
            put::*,
        },
        post::*,
        queue::{
            get::*,
            post::*,
        },
    },
    journal::{
        credit::*,
        get::*,
        post::*,
    },
    printers::{
        get::*,
        queue::{
            delete::*,
            get::*,
            post::*,
        },
    },
    user::{
        get::*,
        post::*,
        put::*,
        tokens::{
            delete::*,
            get::*,
        },
    },
};

fn cors() -> rocket_cors::Cors
{
    rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "X-Api-Key",
            "Content-Type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
}

#[get("/")]
fn api_reference() -> &'static str { include_str!("../../openapi.yaml") }

fn rocket() -> rocket::Rocket
{
    let mysql_pool = get_mysql_pool(20);

    let redis_pool = get_redis_pool(20, Redis::Dispatcher);

    let dispatcher_queue: TaskQueueClient<DispatcherTask, ()> =
        TaskQueueClient::new("dispatcher", redis_pool);

    let redis_pool = get_redis_pool(20, Redis::Store);

    let redis_store = Store::from(redis_pool);

    let redis_pool = get_redis_pool(4, Redis::Store);

    let redis_share = Share::from(redis_pool);

    let mut worker_queues: HashMap<u32, TaskQueueClient<WorkerTask, WorkerCommand>> =
        HashMap::new();

    let connection = mysql_pool.get().expect("getting mysql connection from pool");

    let redis_pool = get_redis_pool(20, Redis::Worker);

    for device_id in select_device_ids(&connection).expect("selecting device ids") {
        let pool = redis_pool.clone();
        worker_queues
            .insert(device_id, TaskQueueClient::new(&format!("worker::{}", device_id), pool));
    }
    let mmdb_reader =
        maxminddb::Reader::open_readfile("GeoLite2-City.mmdb").expect("opening maxminddb reader");

    let redis_pool = get_redis_pool(5, Redis::Lock);

    rocket::ignite()
        .manage(mysql_pool)
        .manage(redis_pool)
        .manage(redis_store)
        .manage(redis_share)
        .manage(mmdb_reader)
        .manage(dispatcher_queue)
        .manage(worker_queues)
        .mount("/", routes![api_reference])
        .mount("/jobs/", routes![
            jobs,
            update_options,
            fetch_options,
            fetch_single_option,
            fetch_info,
            update_filename,
            get_dispatcher_queue,
            upload_job,
            delete_job,
            fetch_job,
            fetch_pdf,
            fetch_preview_0,
            fetch_preview_1,
            fetch_preview_2,
            fetch_preview_3,
            get_sharecode,
            post_sharecode,
        ])
        .mount("/user", routes![
            get_user_info,
            get_user_default_options,
            update_user_default_options,
            login,
            logout,
            credit_redirect,
            change_password,
            fetch_username,
            change_username,
            register_as_new_user,
        ])
        .mount("/user/tokens", routes![
            get_all_tokens,
            delete_all_tokens,
            get_single_token,
            delete_single_token,
        ])
        .mount("/printers", routes![post_to_queue, get_queue, delete_queue])
        .mount("/journal", routes![get_journal_as_user, post_to_journal_with_token, credit])
        .mount("/admin", routes![
            get_printers,
            get_single_printer,
            post_admin_token,
            get_journal_as_admin,
            get_dispatcher_queue_as_admin,
            get_journal_tokens_as_admin,
            post_to_journal_as_admin,
            post_journal_token_as_admin,
            delete_queue_as_admin,
            get_queue_as_admin,
            post_new_admin,
            change_user_password_as_admin,
            change_user_name_as_admin,
            get_user_as_admin,
            get_user_credit_as_admin,
            get_user_journal_as_admin,
            get_all_users,
            change_user_locked,
        ])
        .attach(cors())
}
fn main()
{
    Logger::init().expect("initialising Logger");

    rocket().launch();
}
