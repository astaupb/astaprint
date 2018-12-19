use diesel::{
    prelude::*,
    update,
};
use crate::schema::*;

pub fn update_journal_token(id: u32, used: bool, user_id: u32) -> QueryResult<usize>
{
    update(
        journal_tokens::table
            .filter(journal_tokens::id.eq(token.id))
        )
        .set((
            journal_tokens::used.eq(used),
            journal_tokens::used_by.eq(user_id))
        )
        .execute(connection)
}