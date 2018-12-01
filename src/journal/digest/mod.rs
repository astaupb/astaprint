use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
pub mod table;
use self::table::*;
#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal_digest"]
pub struct JournalDigest
{
    pub id: u32,
    pub digest: Vec<u8>,
    pub credit: BigDecimal,
    pub created: NaiveDateTime,
}
