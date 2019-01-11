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
        pdf -> Longblob,
        pdf_bw -> Longblob,
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
        counter_id -> Unsigned<Integer>,
        control_id -> Unsigned<Integer>,
        status_id -> Unsigned<Integer>,
        info_id -> Unsigned<Integer>,
        location -> Varchar,
        has_a3 -> Bool,
        coin_operated -> Bool,
        description -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    printer_control (id) {
        id -> Unsigned<Integer>,
        queue -> Varchar,
        cancel -> Integer,
        clear -> Integer,
        energy -> Varchar,
        wake -> Integer,
        sleep -> Integer,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    printer_counter (id) {
        id -> Unsigned<Integer>,
        total -> Varchar,
        copy_total -> Varchar,
        copy_bw -> Varchar,
        print_total -> Varchar,
        print_bw -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    printer_info (id) {
        id -> Unsigned<Integer>,
        model -> Varchar,
        hostname -> Varchar,
        location -> Varchar,
        mac -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    printer_status (id) {
        id -> Unsigned<Integer>,
        uptime -> Varchar,
        scan -> Varchar,
        copy -> Varchar,
        toner -> Varchar,
        tray_1 -> Varchar,
        tray_2 -> Varchar,
        tray_3 -> Varchar,
        tray_4 -> Varchar,
        tray_5 -> Varchar,
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
        options -> Nullable<Binary>,
        card -> Nullable<Unsigned<Bigint>>,
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
    printer_control,
    printer_counter,
    printer_info,
    printer_status,
    user,
    user_tokens,
);
