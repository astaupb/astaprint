table! {
    admin (id) {
        id -> Unsigned<Integer>,
        first_name -> Varchar,
        last_name -> Varchar,
        login -> Varchar,
        hash -> Binary,
        salt -> Binary,
        service -> Bool,
        locked -> Bool,
        expires -> Date,
        created_by -> Nullable<Unsigned<Integer>>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    admin_tokens (id) {
        id -> Unsigned<Integer>,
        admin_id -> Unsigned<Integer>,
        user_agent -> Varchar,
        ip -> Varchar,
        location -> Varchar,
        hash -> Binary,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    jobs (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        info -> Varbinary,
        options -> Varbinary,
        pdf -> Longblob,
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
        credit -> Integer,
        value -> Integer,
        print_id -> Nullable<Unsigned<Integer>>,
        admin_id -> Nullable<Unsigned<Integer>>,
        description -> Varchar,
        created -> Timestamp,
    }
}

table! {
    journal_tokens (id) {
        id -> Unsigned<Integer>,
        value -> Unsigned<Integer>,
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
        location -> Varchar,
        has_a3 -> Bool,
        coin_operated -> Bool,
        description -> Varchar,
        watch_toner -> Bool,
        watch_tray1 -> Bool,
        watch_tray2 -> Bool,
        watch_tray3 -> Bool,
        watch_interval -> Unsigned<Integer>,
        last_watched -> Timestamp,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    printer_watchers (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        email -> Varchar,
    }
}

table! {
    print_journal (id) {
        id -> Unsigned<Integer>,
        job_id -> Unsigned<Integer>,
        pages -> Unsigned<Smallint>,
        colored -> Unsigned<Smallint>,
        score -> Smallint,
        device_id -> Unsigned<Integer>,
        options -> Varbinary,
        created -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        hash -> Binary,
        salt -> Binary,
        credit -> Integer,
        options -> Nullable<Binary>,
        card -> Nullable<Unsigned<Bigint>>,
        pin -> Nullable<Unsigned<Integer>>,
        locked -> Bool,
        email -> Nullable<Varchar>,
        tou_accept -> Bool,
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
        updated -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    admin,
    admin_tokens,
    jobs,
    journal,
    journal_tokens,
    printers,
    print_journal,
    user,
    user_tokens,
);
