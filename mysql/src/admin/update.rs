use diesel::{
    prelude::*,
    update,
};
use crate::schema::*;

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
