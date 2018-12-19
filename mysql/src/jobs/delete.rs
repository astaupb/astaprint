use diesel::{
    prelude::*,
    delete,
};
use crate::schema::*;

pub fn delete_job_by_id(id: u32, user_id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    delete(jobs::table.filter(jobs::user_id.eq(user_id)).filter(jobs::id.eq(id))).execute(connection)
}