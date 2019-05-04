use diesel::{
    prelude::*,
    insert_into,
};
use crate::journal::*;

pub fn insert_into_journal(user_id: u32, value: i32, description: &str, connection: &MysqlConnection) -> QueryResult<usize>
{
    insert_into(journal::table)
        .values((
            journal::user_id.eq(user_id),
            journal::value.eq(value),
            journal::description.eq(description),
        ))
        .execute(connection)
}

pub fn insert_into_journal_digest(digest: Vec<u8>, credit: i32, connection: &MysqlConnection) -> QueryResult<usize>
{
    insert_into(journal_digest::table)
        .values((
            journal_digest::digest.eq(digest),
            journal_digest::credit.eq(credit),
        ))
        .execute(connection)
}

pub fn insert_into_journal_token(value: u32, content: String, used: bool, connection: &MysqlConnection) -> QueryResult<usize>
{
    insert_into(journal_tokens::table)
        .values((
            journal_tokens::value.eq(value),
            journal_tokens::content.eq(content),
            journal_tokens::used.eq(used)
        ))
        .execute(connection)
}
