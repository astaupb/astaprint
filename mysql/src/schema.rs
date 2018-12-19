table! {
    admin (id) {
        id -> Unsigned<Integer>,
        first_name -> Varchar,
        last_name -> Varchar,
        hash -> Binary,
        salt -> Binary,
        is_service -> Bool,
        expires -> Date,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    admin_tokens (id) {
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
    jobs (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        info -> Varbinary,
        options -> Varbinary,
        data -> Longblob,
        preview_0 -> Mediumblob,
        preview_1 -> Nullable<Mediumblob>,
        preview_2 -> Nullable<Mediumblob>,
        preview_3 -> Nullable<Mediumblob>,
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
    journal_tokens (id) {
        id -> Unsigned<Integer>,
        value -> Decimal,
        content -> Varchar,
        used -> Bool,
        used_by -> Nullable<Unsigned<Integer>>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    printers (id) {
        id -> Unsigned<Integer>,
        hostname -> Varchar,
        ip -> Varchar,
        community -> Varchar,
        mac -> Varchar,
        device_id -> Unsigned<Integer>,
        objects_id -> Unsigned<Integer>,
        location -> Varchar,
        has_a3 -> Bool,
        coin_operated -> Bool,
        description -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    printer_objects (id) {
        id -> Unsigned<Integer>,
        counter_total -> Varchar,
        counter_copy_total -> Varchar,
        counter_copy_bw -> Varchar,
        counter_print_total -> Varchar,
        counter_print_bw -> Varchar,
        queue_ctl -> Varchar,
        cancel -> Integer,
        clear -> Integer,
        energy_ctl -> Varchar,
        wake -> Integer,
        sleep -> Integer,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        hash -> Binary,
        salt -> Binary,
        card -> Nullable<Binary>,
        pin -> Nullable<Unsigned<Integer>>,
        locked -> Bool,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    user_tokens (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        user_agent -> Varchar,
        ip -> Varchar,
        location -> Varchar,
        hash -> Binary,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    admin,
    admin_tokens,
    jobs,
    journal,
    journal_digest,
    journal_tokens,
    printers,
    printer_objects,
    user,
    user_tokens,
);
