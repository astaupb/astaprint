use std::{
    fs::File,
    str::FromStr,
};

use rocket::{
    response::status::{
        BadRequest,
        Reset,
    },
    State,
};
use rocket_contrib::Json;
use serde_json;

use astaprint::{
    job::{
        data::JobOptions,
        PrintWorkerJSON::{
            self,
            print,
        },
    },
    pagerange::PageRange,
};

use crate::{
    environment::Environment,
    guards::user::User,
    routes::jobs::helper::{
        JobOptionsUpdate,
        Update,
        Value::{
            self,
            B,
            I,
            S,
        },
    },
};

#[get("/<uid>/options/<option>")]

fn fetch_single_option(
    user: User,
    env: State<Environment>,
    uid: String,
    option: String,
) -> Option<Json<Value>>
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
        info!("{} fetched job option {} from {}", user.id, &option, &uid[..8]);

        match option.as_ref() {
            "duplex" => Some(Json(Value::I(u16::from(json.options.duplex)))),
            "copies" => Some(Json(Value::I(json.options.copies))),
            "collate" => Some(Json(Value::B(json.options.collate))),
            "keep" => Some(Json(Value::B(json.options.keep))),
            "a3" => Some(Json(Value::B(json.options.a3))),
            "range" => Some(Json(Value::S(json.options.range))),
            "nup" => Some(Json(Value::I(u16::from(json.options.nup)))),
            "nuppageorder" => Some(Json(Value::I(u16::from(json.options.nuppageorder)))),
            &_ => None,
        }
    } else {
        None
    }
}

#[get("/<uid>/options")]

fn fetch_options(user: User, env: State<Environment>, uid: String) -> Option<Json<JobOptions>>
{
    let path = format!("{}/{}/index/{}", env.userdir, user.id, uid);

    let job = match File::open(&path) {
        Ok(f) => {
            serde_json::from_reader::<File, PrintWorkerJSON>(f)
                .expect("deserializing print worker json from index file")
        },
        Err(_) => return None,
    };

    if let print(json) = job {
        info!("{} fetch options from {}", user.id, &uid[..8]);

        Some(Json(json.options))
    } else {
        None
    }
}

#[put("/<uid>/options/<option>", data = "<value>")]

fn update_single_option(
    user: User,
    env: State<Environment>,
    uid: String,
    option: String,
    value: Json<Value>,
) -> Result<Option<Reset>, BadRequest<String>>
{
    let path = format!("{}/{}/index/{}", env.userdir, user.id, uid);

    let job = match File::open(&path) {
        Ok(f) => {
            serde_json::from_reader::<File, PrintWorkerJSON>(f)
                .expect("serializing print worker json from index file")
        },
        Err(_) => return Ok(None),
    };

    let job = match job {
        print(mut json) => {
            match (option.as_ref(), value.into_inner()) {
                ("duplex", I(value)) => {
                    json.options.duplex = value as u8;
                },
                ("copies", I(value)) => {
                    json.options.copies = value;
                },
                ("collate", B(value)) => {
                    json.options.collate = value;
                },
                ("keep", B(value)) => {
                    json.options.keep = value;
                },
                ("a3", B(value)) => {
                    json.options.a3 = value;
                },
                ("range", S(value)) => {
                    if PageRange::from_str(&value).is_ok() {
                        json.options.range = value;
                    }
                },
                ("nup", I(value)) => {
                    json.options.nup = value as u8;
                },
                ("nuppageorder", I(value)) => {
                    json.options.nuppageorder = value as u8;
                },
                (option, _) => {
                    return Err(BadRequest(Some(format!("{} is unknown or of the wrong type", option))));
                },
            }

            PrintWorkerJSON::print(json)
        },
        _ => return Ok(None),
    };

    match File::create(&path) {
        Ok(f) => {
            serde_json::to_writer::<File, PrintWorkerJSON>(f, &job)
                .expect("serializing new print worker json to index file");

            info!("{} updated {} from {}", user.id, &option, &uid[..8]);

            Ok(Some(Reset))
        },
        Err(_) => Ok(None),
    }
}

#[put("/<uid>/options", data = "<options>")]

fn update_options(
    user: User,
    env: State<Environment>,
    uid: String,
    options: Json<JobOptionsUpdate>,
) -> Option<Reset>
{
    let path = format!("{}/{}/index/{}", env.userdir, user.id, uid);

    let job = match File::open(&path) {
        Ok(f) => {
            serde_json::from_reader::<File, PrintWorkerJSON>(f)
                .expect("serializing print worker json from index file")
        },
        Err(_) => return None,
    };

    let job = match job {
        print(mut json) => {
            json.options = json.options.merge(options.into_inner());

            PrintWorkerJSON::print(json)
        },
        _ => return None,
    };

    match File::create(&path) {
        Ok(f) => {
            info!("{} updated options for {}", user.id, &uid[..8]);

            serde_json::to_writer::<File, PrintWorkerJSON>(f, &job)
                .expect("serializing new print worker json to index file");
        },
        Err(_) => return None,
    }

    Some(Reset)
}
