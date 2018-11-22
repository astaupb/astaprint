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
    io,
};

use rocket::{
    response::status::{
        Accepted,
        BadRequest,
    },
    http::{
        ContentType,
        uncased::UncasedStr,
    },
    State,
};

use astacrypto::random_bytes;

use jobs::{
    info::JobInfo,
    task::{
        DispatcherTask,
        DispatchType,
    },
    uid::UID,
};

use user::guard::UserGuard;

use taskqueue::TaskQueue;


#[derive(FromForm, Debug)]
pub struct UploadForm
{
    pub filename: Option<String>,
    pub password: Option<String>,
    pub color: Option<bool>,
}

#[post("/?<options>", data = "<data>")]
fn upload_job<'a>(
    user: UserGuard,
    data: Vec<u8>,
    options: UploadForm,
    content_type: &ContentType,
    taskqueue: State<TaskQueue<DispatcherTask, ()>>,
) -> Result<Result<Accepted<String>, BadRequest<&'a str>>, io::Error>
{
    if data.len() == 0 {
        return Ok(Err(BadRequest(Some("empty body"))));
    }
    if content_type.top() != UncasedStr::new("application") {
        return Ok(Err(BadRequest(Some("invalid top mime type"))));
    }
    let content = match content_type.sub().as_str() {
        "pdf" => {
            // TODO  check body data for valid header
            DispatchType::PDF
        },
        "zip" => {
            // TODO check for valid zip file ?!
            DispatchType::XPS
        }
        _ => {
            return Ok(Err(BadRequest(Some("invalid sub mime type"))));
        },
    };

    let uid = UID::from(random_bytes(20));

    let info = JobInfo::new(
        &options.filename.unwrap_or_else(|| String::from("")),
        &options.password.unwrap_or_else(|| String::from("")),
        options.color.unwrap_or(false),
    );

    let uid_response = format!("{:x}", uid);

    let task = DispatcherTask {
        user_id: user.id,
        info,
        data,
        uid: uid.bytes,
        content,
    };

    taskqueue.send(&task).expect("sending task to queue");

    info!("{} uploaded job with uid {}", user.id, uid_response);

    Ok(Ok(Accepted(Some(uid_response))))
}
