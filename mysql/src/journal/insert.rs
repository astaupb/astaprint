use diesel::prelude::*;
use bigdecimal::BigDecimal;

pub fn insert_into_journal(user_id: u32, value: BigDecimal, description: &str, connection: &MysqlConnection) -> QueryResult<usize>
{
    insert_into(journal::table)
        .values((
            journal::user_id.eq(user_id),
            journal::value.eq(value),
            journal::description.eq(description),
        ))
        .execute(&mysql)
}

pub fn insert_into_journal_digest(digest: Vec<u8>, credit: BigDecimal, connection: &MysqlConnection) -> QueryResult<usize>
{
    insert_into(journal_digest::table)
        .values((
            journal_digest::digest.eq(digest),
            journal_digest::credit.eq(credit),
        ))
        .execute(&connection)
}
