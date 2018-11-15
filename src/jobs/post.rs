/// AStAPrint-Backend - Jobs POST Routes
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
    collections::HashMap,
    io,
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

use astacrypto::random_bytes;

use jobs::{
    data::JobInfo,
    task::DispatcherTask,
};

use taskqueue::TaskQueue;

use guards::user::UserGuard;

#[derive(Debug, Clone)]
pub struct UID
{
    bytes: Vec<u8>,
}

impl From<Vec<u8>> for UID
{
    fn from(bytes: Vec<u8>) -> UID
    {
        UID {
            bytes,
        }
    }
}

#[derive(FromForm, Debug)]
pub struct UploadForm
{
    pub filename: Option<String>,
    pub password: Option<String>,
    pub color: Option<bool>,
}

#[post("/?<options>", data = "<data>", format = "application/pdf")]
fn upload_job<'a>(
    user: UserGuard,
    data: Vec<u8>,
    options: UploadForm,
    taskqueue: State<TaskQueue<DispatcherTask>>,
) -> Result<Result<Accepted<Json<String>>, BadRequest<&'a str>>, io::Error>
{
    let uid = UID::from(random_bytes(20));

    let info = JobInfo::new(
        &options.filename.unwrap_or_else(|| String::from("")),
        &options.password.unwrap_or_else(|| String::from("")),
        options.color.unwrap_or(false),
    );

    let task = DispatcherTask {
        info,
        data,
    };

    taskqueue.send(&task);

    info!("{} uploaded job with uid {:?}", user.id, uid);

    Ok(Ok(Accepted(Some(Json(format!("{:?}", uid))))))
}
