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

use poppler::PopplerDocument;

use model::task::dispatcher::DispatcherTask;

use jobs::queue::data::PdfBody;
use user::guard::UserGuard;

use redis::{
    queue::TaskQueueClient,
    store::Store,
};


#[post("/queue?<filename>&<keep>&<a3>&<color>&<duplex>&<password>", data = "<data>", format = "application/pdf")]
pub fn upload_job<'a>(
    user: UserGuard,
    data: PdfBody,
    filename: Option<String>,
    keep: Option<bool>,
    a3: Option<bool>,
    color: Option<bool>,
    duplex: Option<u8>,
    password: Option<String>,
    taskqueue: State<TaskQueueClient<DispatcherTask, ()>>,
    store: State<Store>,
) -> Result<Accepted<Json<String>>, BadRequest<&'a str>>
{
    debug!("password: {:?}", password);
    let bytes = data.bytes;

    if let Err(_) = PopplerDocument::new_from_data(&bytes[..], "") {
        return Err(BadRequest(Some("invalid pdf file")));
    }

    let uid = store.set(bytes).expect("saving file in store");

    let hex_uid = hex::encode(&uid[..]);

    let user_id = user.id;

    let filename = if let Some(filename) = filename {
        if filename.len() < 80 {
            filename
        }
        else {
            format!("{}...", &filename[.. 79])
        }
    }
    else {
        String::from("")
    };

    let keep = if let Some(keep) = keep {
        keep
    }
    else {
        false
    };

    let a3 = if let Some(a3) = a3 { a3 } else { false};

    let color = if let Some(color) = color { color } else { false};

    let duplex = if let Some(duplex) = duplex { duplex } else { 0};

    let task = DispatcherTask {
        user_id,
        uid,
        filename,
        keep,
        a3,
        color,
        duplex,
    };

    taskqueue.send(&task).expect("sending task to queue");

    info!("{} uploaded job with uid {}", user.id, hex_uid);

    Ok(Accepted(Some(Json(hex_uid))))
}
