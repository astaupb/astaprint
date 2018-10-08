/// AStAPrint-Database - Printer Schema
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
    printer (id) {
        id -> Unsigned<Smallint>,
        hostname -> Varchar,
        ip -> Varchar,
        community -> Varchar,
        mac -> Varchar,
        device_id -> Unsigned<Smallint>,
        model_id -> Unsigned<Smallint>,
        location -> Varchar,
        description -> Varchar,
    }
}

table! {
    model (id) {
        id -> Unsigned<Smallint>,
        counter_id -> Unsigned<Smallint>,
        queue_ctl_id -> Unsigned<Smallint>,
        energy_ctl_id -> Unsigned<Smallint>,
        name -> Varchar,
    }
}

table! {
    counter (id) {
        id -> Unsigned<Smallint>,
        total -> Varchar,
        print_black -> Varchar,
        print_color -> Nullable<Varchar>,
        copy_black -> Varchar,
        copy_color -> Nullable<Varchar>,
        description -> Varchar,
    }
}

table! {
    energy_ctl (id) {
        id -> Unsigned<Smallint>,
        oid -> Varchar,
        wake -> Integer,
        sleep -> Integer,
    }
}

table! {
    queue_ctl (id) {
        id -> Unsigned<Smallint>,
        oid -> Varchar,
        cancel -> Integer,
        clear -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(printer, model, counter, energy_ctl, queue_ctl,);
