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
use rocket::{
    response::status::{
        Accepted,
        BadRequest,
    },
    State,
};

use rocket_contrib::json::Json;

use redis::{
    queue::TaskQueueClient,
    store::Store,
};

use poppler::PopplerDocument;

use model::task::dispatcher::{
    DispatcherTask,
    DispatcherTaskResponse,
};

use pdf::process::decrypt_pdf_from_data;

use crate::{
    jobs::queue::{
        data::PdfBody,
        start_dispatch,
    },
    user::guard::UserGuard,
};

#[get("/")]
pub fn get_dispatcher_queue(
    user: UserGuard,
    queue: State<TaskQueueClient<DispatcherTask, ()>>,
) -> Option<Json<Vec<DispatcherTaskResponse>>>
{
    Some(Json(
        queue
            .get_processing()
            .iter()
            .filter(|element| element.user_id == user.id)
            .map(|element| (*element).clone())
            .map(|task| DispatcherTaskResponse::from(&task))
            .collect(),
    ))
}

#[post(
    "/?<filename>&<preprocess>&<keep>&<a3>&<color>&<duplex>&<copies>&<password>",
    data = "<data>",
    format = "application/pdf"
)]
pub fn upload_job<'a>(
    user: UserGuard,
    data: PdfBody,
    filename: Option<String>,
    preprocess: Option<u8>,
    keep: Option<bool>,
    a3: Option<bool>,
    color: Option<bool>,
    duplex: Option<u8>,
    copies: Option<u16>,
    password: Option<String>,
    taskqueue: State<TaskQueueClient<DispatcherTask, ()>>,
    store: State<Store>,
) -> Result<Accepted<Json<String>>, BadRequest<&'a str>>
{
    let mut bytes = data.bytes;

    if let Some(password) = password {
        bytes = if let Ok(bytes) = decrypt_pdf_from_data(bytes, &password) {
            bytes
        }
        else {
            return Err(BadRequest(Some("wrong password")))
        }
    }

    if let Err(e) = PopplerDocument::new_from_data(&mut bytes[..], "") {
        info!("Err creating PopplerDocument: {}", e);
        return Err(BadRequest(Some("invalid pdf file")))
    }

    let hex_uid = start_dispatch(
        user.id,
        bytes,
        filename,
        preprocess,
        keep,
        a3,
        color,
        duplex,
        copies,
        store.inner(),
        taskqueue.inner(),
    );

    Ok(Accepted(Some(Json(hex_uid))))
}
