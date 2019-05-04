use crate::{
    journal::*,
    schema::*,
};
use diesel::prelude::*;

pub fn select_journal(connection: &MysqlConnection)
    -> QueryResult<Vec<Journal>>
{
    journal::table.select(journal::all_columns).load(connection)
}

pub fn select_journal_of_user(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Vec<Journal>>
{
    journal::table
        .select(journal::all_columns)
        .filter(journal::user_id.eq(user_id))
        .order(journal::id.desc())
        .load(connection)
}

pub fn select_latest_journal_entry(
    connection: &MysqlConnection,
) -> QueryResult<Journal>
{
    journal::table
        .select(journal::all_columns)
        .order(journal::id.desc())
        .first(connection)
}

pub fn select_journal_digest(
    connection: &MysqlConnection,
) -> QueryResult<Vec<JournalDigest>>
{
    journal_digest::table.select(journal_digest::all_columns).load(connection)
}

pub fn select_latest_journal_digest(
    connection: &MysqlConnection,
) -> QueryResult<JournalDigest>
{
    journal_digest::table
        .select(journal_digest::all_columns)
        .order(journal_digest::id.desc())
        .first(connection)
}


pub fn select_journal_tokens(
    connection: &MysqlConnection,
) -> QueryResult<Vec<JournalToken>>
{
    journal_tokens::table.select(journal_tokens::all_columns).load(connection)
}

pub fn select_latest_journal_id_of_user(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<Option<u32>>
{
    user::table
        .inner_join(journal::table)
        .select(journal::id)
        .filter(user::id.eq(journal::user_id))
        .filter(user::id.eq(user_id))
        .order(journal::id.desc())
        .first(connection)
        .optional()
}

pub fn select_credit_by_id(
    id: u32,
    connection: &MysqlConnection,
) -> QueryResult<i32>
{
    journal_digest::table
        .select(journal_digest::credit)
        .filter(journal_digest::id.eq(id))
        .first(connection)
}

pub fn select_journal_token_by_content(content: String, connection: &MysqlConnection) -> QueryResult<Option<JournalToken>>
{
    journal_tokens::table
        .select(journal_tokens::all_columns)
        .filter(journal_tokens::content.eq(content))
        .first(connection)
        .optional()
}
