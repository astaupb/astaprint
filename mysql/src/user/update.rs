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
