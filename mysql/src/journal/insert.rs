use diesel::{
    prelude::*,
    insert_into,
};
use crate::journal::*;

pub fn insert_into_journal(user_id: u32, credit: i32, value: i32, print_id: Option<u32>, admin_id: Option<u32>, description: &str, connection: &MysqlConnection) -> QueryResult<usize>
{
    insert_into(journal::table)
        .values((
            journal::user_id.eq(user_id),
            journal::credit.eq(credit),
            journal::value.eq(value),
            journal::print_id.eq(print_id),
            journal::admin_id.eq(admin_id),
            journal::description.eq(description),
        ))
        .execute(connection)
}

pub fn insert_into_print_journal(job_id: u32, pages: u16, colored: u16, score: i16, device_id: u32, options: Vec<u8>, connection: &MysqlConnection) -> QueryResult<usize>
{
    insert_into(print_journal::table)
        .values((
            print_journal::job_id.eq(job_id),
            print_journal::pages.eq(pages),
            print_journal::colored.eq(colored),
            print_journal::score.eq(score),
            print_journal::device_id.eq(device_id),
            print_journal::options.eq(options),
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
