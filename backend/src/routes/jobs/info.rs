use rocket::State;
use rocket_contrib::Json;
use serde_json;
use std::fs::File;

use crate::{
    environment::Environment,
    guards::user::User,
};

use astaprint::job::{
    data::JobInfo,
    PrintWorkerJSON::{
        self,
        print,
    },
};

use super::helper::Value;

#[get("/<uid>/info/<info>")]

fn fetch_single_info(user: User, env: State<Environment>, uid: String, info: String)
    -> Option<Json<Value>>
{
    let path = format!("{}/{}/index/{}", env.userdir, user.id, uid);

    let job = match File::open(&path) {
        Ok(f) => {
            serde_json::from_reader::<File, PrintWorkerJSON>(f)
                .expect("serializing print worker json from index file")
        },
        Err(_) => return None,
    };

    if let print(json) = job {
        info!("{} fetched info {} from {}", user.id, &info, &uid[..8]);

        match info.as_ref() {
            "filename" => Some(Json(Value::S(json.info.filename))),
            "pagecount" => Some(Json(Value::I(json.info.pagecount))),
            "color" => Some(Json(Value::B(json.info.color))),
            "a3" => Some(Json(Value::B(json.info.a3))),
            "password" => Some(Json(Value::S(json.info.password))),
            &_ => None,
        }
    } else {
        None
    }
}

#[get("/<uid>/info")]

fn fetch_info(user: User, env: State<Environment>, uid: String) -> Option<Json<JobInfo>>
{
    let path = format!("{}/{}/index/{}", env.userdir, user.id, uid);

    let job = match File::open(&path) {
        Ok(f) => {
            serde_json::from_reader::<File, PrintWorkerJSON>(f)
                .expect("serializing print worker json from index file")
        },
        Err(_) => return None,
    };

    if let print(json) = job {
        info!("{} fetched job infos from {}", user.id, &uid[..8]);

        Some(Json(json.info))
    } else {
        None
    }
}
