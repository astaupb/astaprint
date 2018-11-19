#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
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
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate base64;
extern crate logger;

extern crate diesel;

extern crate astaprint;
extern crate taskqueue;

use std::env;

use diesel::{
    prelude::MysqlConnection,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders,
    AllowedOrigins,
};

use taskqueue::{
    create_pool,
    TaskQueue,
};

use logger::Logger;

use astaprint::{
    jobs::{
        post::*,
        get::*,
        delete::*,
        task::DispatcherTask,
        options::{
            get::*,
            put::*,
        },
        info::{
            get::*,
        },
    },
    user::http::{
        tokens::*,
        *,
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
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "X-Api-Key", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
}

use rocket::response::Stream;
use std::process::{
    ChildStdout,
    Command,
    Stdio,
};

#[get("/")]
fn api_reference() -> Stream<ChildStdout>
{
    let curl = Command::new("curl")
        .arg("https://git.uni-paderborn.de/asta/astaprint-docs/raw/master/api_reference.yml")
        .stdout(Stdio::piped())
        .spawn()
        .expect("get current api specification with curl from git.upb.de");

    Stream::from(curl.stdout.unwrap())
}

fn rocket() -> rocket::Rocket
{
    let url = env::var("ASTAPRINT_DATABASE_URL").expect("reading ASTAPRINT_DATABASE_URL from environment");

    let manager = ConnectionManager::<MysqlConnection>::new(url);

    let mariadb_pool = Pool::new(manager).expect("initiliasing MySQL Pool");

    let url = env::var("ASTAPRINT_REDIS_URL").expect("reading ASTAPRINT_REDIS_URL from environment");

    let redis_pool = create_pool(&url);

    let dispatcher_queue: TaskQueue<DispatcherTask> = TaskQueue::new("dispatcher", redis_pool);

    rocket::ignite()
        .manage(mariadb_pool)
        .manage(dispatcher_queue)
        .mount("/", routes![api_reference])
        .mount(
            "/jobs/",
            routes![
                jobs,
                update_options,
                update_single_option,
                fetch_options,
                fetch_single_option,
                fetch_info,
                //fetch_single_info,
                // -> do we really need this?
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
            routes![get_user_info, login, logout, change_password, credit, fetch_username, change_username],
        )
        .mount(
            "/user/tokens",
            routes![get_all_tokens, delete_all_tokens, get_single_token, delete_single_token],
        )
        /*
        .mount("/printers", routes![print_job])
        .mount("/register", routes![register])
        .mount("/journal", routes![journal])
        */
        .attach(cors())
}

fn main()
{
    Logger::init("backend").expect("initialising Logger");

    rocket().launch();
}
