/// AStAPrint-Backend - Jobs Routes
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
use std::{
    fs::{
        read_dir,
        File,
    },
    io::{
        self,
        Write,
    },
};

use rocket::{
    response::{
        status::{
            Accepted,
            BadRequest,
            Reset,
        },
        NamedFile,
    },
    State,
};
use rocket_contrib::Json;
use serde_json;

use json_receiver::JsonReceiver;

use crate::{
    crypto::urandom,
    environment::Environment,
    guards::user::User,
};

use astaprint::job::{
    DispatchWorkerJSON,
    JobData,
    JobFiles,
    PrintWorkerJSON::{
        self,
        cancel,
        print,
    },
};

pub mod helper;
pub mod info;
pub mod options;

use crate::routes::jobs::helper::{
    Hex,
    UploadForm,
};

#[get("/<uid>")]

fn fetch_job(user: User, env: State<Environment>, uid: String) -> Option<Json<JobData>>
{
    let path = format!("{}/{}/index/{}", env.userdir, user.id, uid);

    match File::open(&path) {
        Ok(file) => {
            println!("serializing");

            match serde_json::from_reader::<File, PrintWorkerJSON>(file) {
                Ok(json) => {
                    match json {
                        print(job) => {
                            info!("{} fetched job {}", user.id, &uid[..8]);

                            Some(Json(job))
                        },
                        cancel(_) => None,
                    }
                },
                Err(_) => None,
            }
        },
        Err(_) => None,
    }
}

#[delete("/<uid>")]

fn delete_job(user: User, uid: String) -> Option<Reset>
{
    let files = JobFiles::new(&uid, user.id);

    let job = match File::open(&files.index) {
        Ok(f) => {
            serde_json::from_reader::<File, PrintWorkerJSON>(f)
                .expect("serializing print worker json from index file")
        },
        Err(_) => return None,
    };

    if let print(json) = job {
        files.clean_up(json.info.pagecount);

        info!("{} deleted job {}", user.id, &uid[..8]);

        Some(Reset)
    } else {
        return None;
    }
}

#[get("/<uid>/pdf")]

fn fetch_pdf(user: User, env: State<Environment>, uid: String) -> Option<NamedFile>
{
    let path = format!("{}/{}/pdf/{}", env.userdir, user.id, uid);

    info!("{} fetched pdf from {}", user.id, &uid[..8]);

    NamedFile::open(&path).ok()
}

#[get("/<uid>/preview/<index>")]

fn fetch_preview(user: User, env: State<Environment>, uid: String, index: String) -> Option<NamedFile>
{
    let path = format!("{}/{}/preview/{}-{}", env.userdir, user.id, uid, index);

    info!("{} fetched preview #{} from {}", user.id, index, &uid[..8]);

    NamedFile::open(&path).ok()
}

#[get("/")]

fn jobs(user: User, env: State<Environment>) -> Json<Vec<JobData>>
{
    let index_root = format!("{}/{}/index/", env.userdir, user.id);

    info!("{} fetched jobs", user.id);

    Json(
        read_dir(index_root)
            .expect("reading job index directory")
            .map(|read| read.unwrap().path())
            .filter_map(|path| {
                let f = File::open(path).unwrap();
                serde_json::from_reader::<File, PrintWorkerJSON>(f).ok()
            })
            .filter_map(|json| {
                match json {
                    print(job) => Some(job),
                    cancel(_) => None,
                }
            })
            .collect(),
    )
}

#[post("/?<options>", data = "<file>", format = "application/pdf")]

fn upload_job<'a>(
    user: User,
    env: State<Environment>,
    file: Vec<u8>,
    options: UploadForm,
    dispatch: State<JsonReceiver<DispatchWorkerJSON>>,
) -> Result<Result<Accepted<Json<String>>, BadRequest<&'a str>>, io::Error>
{
    let mut uid: [u8; 32] = [0; 32];

    urandom(&mut uid[..]);

    let uid = uid[..].to_hex();

    let mut tmp_file = File::create(&format!("{}/{}/tmp/{}", env.userdir, user.id, uid))?;

    tmp_file.write_all(&file[..])?;

    let job = JobData::new(
        &uid,
        user.id,
        &options.filename.unwrap_or_else(|| String::from("")),
        &options.password.unwrap_or_else(|| String::from("")),
        options.color.unwrap_or(false),
    );

    dispatch.feed(
        DispatchWorkerJSON {
            dispatch: job,
        },
        &uid,
    );

    info!("{} uploaded job with uid {}", user.id, uid);

    Ok(Ok(Accepted(Some(Json(uid)))))
}
