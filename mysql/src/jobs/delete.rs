use diesel::{
    prelude::*,
    delete,
};
use crate::schema::*;

pub fn delete_job_by_id(id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    delete(jobs::table.filter(jobs::id.eq(id))).execute(connection)
}

pub fn delete_job_of_user_by_id(user_id: u32, id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    delete(jobs::table.filter(jobs::id.eq(id)).filter(jobs::user_id.eq(user_id))).execute(connection)
}

pub fn delete_all_jobs_of_user(user_id: u32, connection: &MysqlConnection) -> QueryResult<usize>
{
    delete(jobs::table.filter(jobs::user_id.eq(user_id))).execute(connection)
}
