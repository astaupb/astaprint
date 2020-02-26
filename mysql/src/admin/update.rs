use diesel::{
    prelude::*,
    update,
};
use crate::{
    schema::*,
    admin::Admin,
};


pub fn update_admin(admin: Admin, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(admin::table.filter(admin::id.eq(admin.id)))
        .set((
            admin::first_name.eq(admin.first_name),
            admin::last_name.eq(admin.last_name),
            admin::login.eq(admin.login),
            admin::service.eq(admin.service),
            admin::locked.eq(admin.locked),
            admin::expires.eq(admin.expires),
        ))
        .execute(connection)
}

pub fn update_admin_hash_and_salt_by_id(id: u32, hash: Vec<u8>, salt: Vec<u8>, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(admin::table.filter(admin::id.eq(id))).set((admin::hash.eq(hash), admin::salt.eq(salt))).execute(connection)
}
pub fn update_admin_token(token_id: u32, user_agent: String, ip: String, location: String, connection: &MysqlConnection) -> QueryResult<usize>
{
    update(admin_tokens::table.filter(admin_tokens::id.eq(token_id)))
           .set((
               admin_tokens::user_agent.eq(user_agent),
               admin_tokens::ip.eq(ip),
               admin_tokens::location.eq(location),
           ))
           .execute(connection)
}
