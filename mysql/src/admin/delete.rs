use crate::{
    schema::*,
};
use diesel::prelude::*;

pub fn delete_admin_by_id(id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    diesel::delete(
        admin::table
            .filter(admin::id.eq(id))
        )
        .execute(connection)
}


