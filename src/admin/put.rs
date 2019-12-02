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
use admin::guard::AdminGuard;
use mysql::user::update::*;
use rocket::http::Status;
use rocket_contrib::json::Json;
use diesel::QueryResult;

#[put("/users/<id>/locked", data = "<locked>")]
pub fn change_user_locked(id: u32, locked: Json<bool>, admin: AdminGuard) -> Status
{
    let locked = locked.into_inner();
    match update_locked(id, locked, &admin.connection) {
        Ok(1) => {
            info!("user {} locked: {}", id, locked);
            Status::new(205, "Success - Reset Content")
        },
        err => {
            error!("{:?}", err);
            Status::new(500, "Internal Server Error")
        },
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Card
{
    sn: Option<u64>,
    pin: Option<u32>,
}

#[put("/users/<id>/card", data = "<card>")]
pub fn change_user_card(admin: AdminGuard, id: u32, card: Json<Card>) -> QueryResult<Status>
{
    let card = card.into_inner();

    update_user_card_and_pin(id, card.sn, card.pin, &admin.connection)?;

    Ok(Status::new(205, "Reset Content"))
}

#[put("/users/<id>/email?<email>")]
pub fn change_user_email_as_admin(
    admin: AdminGuard,
    id: u32,
    email: String,
) -> QueryResult<Status>
{
    update_user_email(id, Some(email), &admin.connection)?;

    Ok(Status::new(205, "Reset Content"))
}
