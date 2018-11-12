table! {
    jobs (id) {
        id -> Unsigned<Integer>,
        user_id -> Nullable<Unsigned<Integer>>,
        info -> Binary,
        options -> Binary,
        data -> Longblob,
        preview-0 -> Mediumblob,
        preview-1 -> Nullable<Mediumblob>,
        preview-2 -> Nullable<Mediumblob>,
        preview-3 -> Nullable<Mediumblob>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    journal (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        value -> Decimal,
        description -> Varchar,
        created -> Timestamp,
    }
}

table! {
    journal_digest (id) {
        id -> Unsigned<Integer>,
        digest -> Binary,
        credit -> Decimal,
        created -> Timestamp,
    }
}

table! {
    manager (id) {
        id -> Unsigned<Integer>,
        first_name -> Varchar,
        last_name -> Varchar,
        password_hash -> Binary,
        password_salt -> Binary,
        is_service -> Bool,
        expires -> Date,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    manager_token (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        user_agent -> Varchar,
        ip -> Varchar,
        location -> Varchar,
        hash -> Binary,
        salt -> Binary,
        created -> Timestamp,
    }
}

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

table! {
    printer_energy_ctl (id) {
        id -> Unsigned<Smallint>,
        oid -> Varchar,
        wake -> Integer,
        sleep -> Integer,
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

table! {
    printer_queue_ctl (id) {
        id -> Unsigned<Smallint>,
        oid -> Varchar,
        cancel -> Integer,
        clear -> Integer,
    }
}

table! {
    register_token (id) {
        id -> Unsigned<Smallint>,
        value -> Varchar,
        used -> Bool,
        user_id -> Nullable<Unsigned<Integer>>,
        created -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Unsigned<Integer>,
        username -> Varchar,
        password_hash -> Binary,
        password_salt -> Binary,
        pin -> Nullable<Varchar>,
        locked -> Bool,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    user_token (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        user_agent -> Varchar,
        ip -> Varchar,
        location -> Varchar,
        hash -> Binary,
        salt -> Binary,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    jobs,
    journal,
    journal_digest,
    manager,
    manager_token,
    printers,
    printer_counter,
    printer_energy_ctl,
    printer_model,
    printer_queue_ctl,
    register_token,
    user,
    user_token,
);
