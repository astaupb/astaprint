/// AStAPrint - Jobs GET Routes
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
