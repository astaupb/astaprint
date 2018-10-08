/// AStAPrint-Database - User Schema
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

table! {
    user (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        locked -> Bool,
        pin_hash -> Option<Binary>,
        pin_salt -> Option<Binary>,
        password_hash -> Binary,
        password_salt -> Binary,
        timestamp -> Timestamp,
    }
}

joinable!(journal -> user (user_id));

table! {
    journal (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        value -> Numeric,
        credit -> Numeric,
        description -> Varchar,
        timestamp -> Timestamp,
    }
}

table! {
    journal_digest (id) {
        id -> Unsigned<Integer>,
        digest -> Binary,
        timestamp -> Timestamp,
    }
}

joinable!(token -> user (user_id));

table! {
    token (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        user_agent -> Varchar,
        location -> Varchar,
        value -> Binary,
        timestamp -> Timestamp,
    }
}

joinable!(register_token -> user (user_id));

table! {
    register_token (id) {
        id -> Unsigned<Smallint>,
        value -> Varchar,
        used -> Bool,
        user_id -> Nullable<Unsigned<Integer>>,
        timestamp -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(user, journal, journal_digest, token, register_token,);
