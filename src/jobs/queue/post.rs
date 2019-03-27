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
use std::io;

use rocket::{
    response::status::{
        Accepted,
        BadRequest,
    },
    State,
};

use rocket_contrib::json::Json;

use model::task::dispatcher::DispatcherTask;

use jobs::queue::data::PdfBody;
use user::guard::UserGuard;

use redis::{
    queue::TaskQueueClient,
    store::Store,
};

#[post("/queue?<filename>&<password>", data = "<data>", format = "application/pdf")]
pub fn upload_job<'a>(
    user: UserGuard,
    data: PdfBody,
    filename: Option<String>,
    password: Option<String>,
    taskqueue: State<TaskQueueClient<DispatcherTask, ()>>,
    store: State<Store>,
) -> io::Result<Result<Accepted<Json<String>>, BadRequest<&'a str>>>
{
    if let Some(_password) = password {
        // TODO decrypt with qpdf
    }
    debug!("filename: {:?}", filename);

    let uid = store.set(data.bytes).expect("saving file in store");

    let hex_uid = hex::encode(&uid[..]);

    let user_id = user.id;

    let filename = if let Some(filename) = filename {
        if filename.len() < 80 {
            filename
        } else {
            format!("{}...", &filename[..79])
        }
    }
    else {
        String::from("")
    };

    let task = DispatcherTask {
        user_id, uid, filename,
    };

    taskqueue.send(&task).expect("sending task to queue");

    info!("{} uploaded job with uid {}", user.id, hex_uid);

    Ok(Ok(Accepted(Some(Json(hex_uid)))))
}
