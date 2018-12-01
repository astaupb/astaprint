use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
pub mod table;
use self::table::*;
#[derive(Identifiable, Queryable, Insertable, Associations, Debug)]
#[table_name = "journal_tokens"]
pub struct JournalToken
{
    pub id: u32,
    pub value: BigDecimal,
    pub content: String,
    pub used: bool,
    used_by: Option<u32>,
    created: NaiveDateTime,
    updated: NaiveDateTime,
}
