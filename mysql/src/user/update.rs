use diesel::{
    prelude::*,
    update,
};
use crate::schema::*;

pub fn update_hash_and_salt(user_id: u32, hash: Vec<u8>, salt: Vec<u8>, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(user::table.filter(user::id.eq(user_id)))
        .set((user::hash.eq(hash), user::salt.eq(salt)))
        .execute(connection)
}
pub fn update_user_name(user_id: u32, name: &str, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(user::table.filter(user::id.eq(user_id)))
        .set(user::name.eq(name))
        .execute(connection)
}

pub fn update_user_card_and_pin(user_id: u32, card: Option<u64>, pin: Option<u32>, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(user::table.filter(user::id.eq(user_id)))
        .set((user::card.eq(card), user::pin.eq(pin)))
        .execute(connection)
}

pub fn update_locked(user_id: u32, locked: bool, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(user::table.filter(user::id.eq(user_id)))
        .set(user::locked.eq(locked))
        .execute(connection)
}

pub fn update_user_credit(user_id: u32, credit: i32, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(user::table.filter(user::id.eq(user_id)))
        .set(user::credit.eq(credit))
        .execute(connection)
}

pub fn update_default_job_options(user_id: u32, options: Option<Vec<u8>>, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(user::table.filter(user::id.eq(user_id)))
        .set(user::options.eq(options))
        .execute(connection)
}

pub fn update_user_token(token_id: u32, user_agent: String, ip: String, location: String, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(user_tokens::table.filter(user_tokens::id.eq(token_id)))
           .set((
               user_tokens::user_agent.eq(user_agent),
               user_tokens::ip.eq(ip),
               user_tokens::location.eq(location),
           ))
           .execute(connection)
}
