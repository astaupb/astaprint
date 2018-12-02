/// AStAPrint - Printers Table
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
    printers (id) {
        id -> Unsigned<Smallint>,
        hostname -> Varchar,
        ip -> Varchar,
        community -> Varchar,
        mac -> Varchar,
        device_id -> Unsigned<Smallint>,
        model_id -> Unsigned<Smallint>,
        location -> Varchar,
        description -> Varchar,
        updated -> Timestamp,
    }
}

table! {
    printer_model (id) {
        id -> Unsigned<Smallint>,
        counter_id -> Unsigned<Smallint>,
        queue_ctl_id -> Unsigned<Smallint>,
        energy_ctl_id -> Unsigned<Smallint>,
        description -> Varchar,
    }
}

joinable!(printers -> printer_model (model_id));

table! {
    printer_counter (id) {
        id -> Unsigned<Smallint>,
        total -> Varchar,
        print_black -> Varchar,
        print_color -> Nullable<Varchar>,
        copy_black -> Varchar,
        copy_color -> Nullable<Varchar>,
        description -> Varchar,
    }
}

joinable!(printer_model -> printer_counter (counter_id));

table! {
    printer_queue_ctl (id) {
        id -> Unsigned<Smallint>,
        oid -> Varchar,
        cancel -> Integer,
        clear -> Integer,
    }
}

joinable!(printer_model -> printer_queue_ctl (queue_ctl_id));

table! {
    printer_energy_ctl (id) {
        id -> Unsigned<Smallint>,
        oid -> Varchar,
        wake -> Integer,
        sleep -> Integer,
    }
}

joinable!(printer_model -> printer_energy_ctl (energy_ctl_id));

allow_tables_to_appear_in_same_query!(
    printers,
    printer_model,
    printer_counter,
    printer_energy_ctl,
    printer_queue_ctl,
);
