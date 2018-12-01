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
