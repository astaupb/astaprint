use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

table! {
    journal (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        value -> Decimal,
        description -> Varchar,
        created -> Timestamp,
    }
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal"]
pub struct Journal
{
    pub id: u32,
    pub user_id: u32,
    pub value: BigDecimal,
    pub description: String,
    pub created: NaiveDateTime,
}

table! {
    journal_digest (id) {
        id -> Unsigned<Integer>,
        digest -> Binary,
        credit -> Decimal,
        created -> Timestamp,
    }
}

#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal_digest"]
pub struct JournalDigest
{
    pub id: u32,
    pub digest: Vec<u8>,
    pub credit: BigDecimal,
    pub created: NaiveDateTime,
}
