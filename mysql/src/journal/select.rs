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

pub fn select_print_journal(connection: &MysqlConnection) -> QueryResult<Vec<PrintJournal>>
{
    print_journal::table.select(print_journal::all_columns).load(connection)
}

pub fn select_journal_with_limit_and_offset(
    limit: i64,
    offset: i64,
    connection: &MysqlConnection,
) -> QueryResult<Vec<Journal>>
{
    journal::table
        .select(journal::all_columns)
        .order(journal::id.desc())
        .limit(limit)
        .offset(offset)
        .load(connection)
}

pub fn select_journal_of_user_with_limit_and_offset(
    user_id: u32,
    limit: i64,
    offset: i64,
    connection: &MysqlConnection,
) -> QueryResult<Vec<Journal>>
{
    journal::table
        .select(journal::all_columns)
        .filter(journal::user_id.eq(user_id))
        .order(journal::id.desc())
        .limit(limit)
        .offset(offset)
        .load(connection)
}

pub fn select_print_journal_by_id(
    id: u32,
    connection: &MysqlConnection,
) -> QueryResult<PrintJournal>
{
    print_journal::table
        .filter(print_journal::id.eq(id))
        .first(connection)
}

pub fn select_latest_credit_of_user(
    user_id: u32,
    connection: &MysqlConnection,
) -> QueryResult<i32>
{
    journal::table
        .select(journal::credit)
        .filter(journal::user_id.eq(user_id))
        .order(journal::id.desc())
        .first(connection)
}

pub fn select_latest_print_journal_id(
    connection: &MysqlConnection,
) -> QueryResult<u32>
{
    print_journal::table
        .select(print_journal::id)
        .order(print_journal::id.desc())
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

pub fn select_journal_token_by_id(id: u32, connection: &MysqlConnection) -> QueryResult<JournalToken>
{
    journal_tokens::table
        .select(journal_tokens::all_columns)
        .filter(journal_tokens::id.eq(id))
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
