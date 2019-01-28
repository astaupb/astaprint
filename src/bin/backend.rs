#![feature(proc_macro_hygiene, decl_macro)]
/// AStAPrint - Backend
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

use std::{
    collections::HashMap,
};

use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
};

use redis::{
    Redis,
    get_redis_pool,
    queue::TaskQueueClient,
    store::Store,
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
        put::*,
        post::*,
        tokens::*,
    },
    jobs::{
        delete::*,
        get::*,
        info::get::*,
        options::{
            get::*,
            put::*,
        },
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
        queue::{
            delete::*,
            get::*,
            post::*,
        },
        get::*,
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
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Delete,
            Method::Put,
        ]
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
fn api_reference() -> &'static str
{
    include_str!("../../openapi.yaml")
}

fn rocket() -> rocket::Rocket
{
    let mysql_pool = get_mysql_pool(20);

    let redis_pool = get_redis_pool(20, Redis::Dispatcher);

    let dispatcher_queue: TaskQueueClient<DispatcherTask, ()> =
        TaskQueueClient::new("dispatcher", redis_pool.clone());

    let redis_store = Store::from(redis_pool);

    let mut worker_queues: HashMap<
        u32,
        TaskQueueClient<WorkerTask, WorkerCommand>,
    > = HashMap::new();

    let connection = mysql_pool.get().expect("getting mysql connection from pool");

    let redis_pool = get_redis_pool(20, Redis::Worker);

    for device_id in select_device_ids(&connection).expect("selecting device ids") {
        let pool = redis_pool.clone();
        worker_queues.insert(
            device_id,
            TaskQueueClient::new(&format!("worker::{}", device_id), pool),
        );
    }
    let mmdb_reader = maxminddb::Reader::open_readfile(
        "GeoLite2-City_20181127/GeoLite2-City.mmdb",
    )
    .expect("opening maxminddb reader");

    let redis_pool = get_redis_pool(5, Redis::Lock);

    rocket::ignite()
        .manage(mysql_pool)
        .manage(redis_pool)
        .manage(redis_store)
        .manage(mmdb_reader)
        .manage(dispatcher_queue)
        .manage(worker_queues)
        .mount("/", routes![api_reference]) .mount("/", routes![get_user_as_admin, get_user_credit_as_admin, get_user_journal_as_admin, get_all_users, change_user_locked,])
        .mount("/", routes![get_printers, get_single_printer, post_admin_token])
        .mount(
            "/jobs/",
            routes![
                jobs,
                update_options,
                update_single_option,
                fetch_options,
                fetch_single_option,
                fetch_info,
                get_dispatcher_queue,
                upload_job,
                delete_job,
                fetch_job,
                fetch_pdf,
                fetch_preview_0,
                fetch_preview_1,
                fetch_preview_2,
                fetch_preview_3,
            ],
        )
        .mount(
            "/user",
            routes![
                get_user_info,
                login,
                logout,
                credit_redirect,
                change_password,
                fetch_username,
                change_username
            ],
        )
        .mount(
            "/user/tokens",
            routes![
                get_all_tokens,
                delete_all_tokens,
                get_single_token,
                delete_single_token
            ],
        )
        .mount("/printers", routes![print_job, get_queue, delete_queue])
        .mount("/journal", routes![get_journal_as_user, credit])
        .mount("/admin", routes![post_admin_token, get_journal_as_admin, post_to_journal_as_admin, delete_queue_as_admin, get_queue_as_admin, post_new_admin])
        .attach(cors())
}
fn main()
{
    Logger::init().expect("initialising Logger");

    rocket().launch();
}
