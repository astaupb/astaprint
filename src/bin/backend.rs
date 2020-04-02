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

use std::{
    collections::HashMap,
    env,
};

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

use model::{
    job::options::update::JobOptionsUpdate,
    task::{
        dispatcher::DispatcherTask,
        worker::{
            WorkerCommand,
            WorkerTask,
        },
    },
};

use logger::Logger;

use astaprint::{
    admin::{
        admins::http::*,
        jobs::http::*,
        journal::http::*,
        printers::http::*,
        tokens::http::*,
        users::http::*,
    },
    jobs::{
        http::*,
        queue::http::*,
    },
    journal::http::*,
    printers::{
        http::*,
        queue::http::*,
    },
    user::{
        http::*,
        tokens::http::*,
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

#[get("/")]
fn api_reference_admin() -> &'static str { include_str!("../../openapi_admin.yaml") }

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

    let mut worker_queues: HashMap<
        u32,
        TaskQueueClient<WorkerTask, WorkerCommand<Option<JobOptionsUpdate>>>,
    > = HashMap::new();

    let connection = mysql_pool.get().expect("getting mysql connection from pool");

    let redis_pool = get_redis_pool(20, Redis::Worker);

    for device_id in select_device_ids(&connection).expect("selecting device ids") {
        let pool = redis_pool.clone();
        worker_queues
            .insert(device_id, TaskQueueClient::new(&format!("worker::{}", device_id), pool));
    }
    let mmdb_reader = maxminddb::Reader::open_readfile(
        &env::var("ASTAPRINT_MMDB_FILE").expect("reading path of mmdb file from env"),
    )
    .expect("opening maxminddb reader");

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
            delete_all_jobs,
            fetch_job,
            delete_job,
            update_options,
            copy_job,
            fetch_pdf,
            fetch_preview_0,
            fetch_preview_1,
            fetch_preview_2,
            fetch_preview_3,
            fetch_info,
            fetch_options,
            get_sharecode,
            post_sharecode,
        ])
        .mount("/jobs/queue", routes![upload_job, get_dispatcher_queue])
        .mount("/user", routes![
            get_user_summary,
            register_as_new_user,
            get_user_default_options,
            update_user_default_options,
            change_user_tou_accept,
            fetch_username,
            change_username,
            change_email,
            change_password,
            fetch_credit,
            logout,
        ])
        .mount("/user/tokens", routes![
            get_all_tokens,
            delete_all_tokens,
            login,
            get_single_token,
            delete_single_token,
        ])
        .mount("/printers", routes![post_to_queue, delete_queue, get_printers, get_single_printer,])
        .mount("/journal", routes![get_journal_as_user, post_to_journal_with_token, credit])
        .mount("/admin", routes![api_reference_admin])
        .mount("/admin/admins", routes![
            get_admins,
            post_new_admin,
            get_single_admin,
            delete_admin,
            put_admin,
            put_admin_password
        ])
        .mount("/admin/jobs", routes![get_dispatcher_queue_as_admin,])
        .mount("/admin/printers", routes![
            get_printers_as_admin,
            post_printer,
            get_single_printer_as_admin,
            delete_printer,
            put_printer_details,
            get_queue_as_admin,
            delete_queue_as_admin,
        ])
        .mount("/admin/journal", routes![
            get_journal_as_admin,
            get_journal_tokens_as_admin,
            post_to_journal_as_admin,
            post_journal_token_as_admin,
        ])
        .mount("/admin/tokens", routes![
            post_admin_token,
            get_admin_tokens,
            get_single_admin_token,
            delete_admin_tokens,
            delete_single_admin_token
        ])
        .mount("/admin/users", routes![
            get_all_users,
            get_user_as_admin,
            get_user_credit_as_admin,
            get_user_journal_as_admin,
            reset_user_password_as_admin,
            change_user_password_as_admin,
            change_user_name_as_admin,
            change_user_card_as_admin,
            change_user_email_as_admin,
            change_user_locked,
            clear_tou_accept,
        ])
        .attach(cors())
}
fn main()
{
    Logger::init().expect("initialising Logger");

    rocket().launch();
}
