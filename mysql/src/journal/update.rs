use diesel::{
    prelude::*,
    update,
};
use crate::schema::*;

pub fn update_journal_token(id: u32, used: bool, user_id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(
        journal_tokens::table
            .filter(journal_tokens::id.eq(id))
        )
        .set((
            journal_tokens::used.eq(used),
            journal_tokens::used_by.eq(user_id))
        )
        .execute(connection)
}

pub fn update_print_journal_options_by_id(id: u32, options: Vec<u8>, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(
        print_journal::table
            .filter(print_journal::id.eq(id))
        )
        .set(
            print_journal::options.eq(options)
        )
        .execute(connection)
}
