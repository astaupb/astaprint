use diesel::{
    prelude::*,
    update,
};

use crate::schema::*;

pub fn update_job_options(
    job_id: u32,
    user_id: u32,
    value: Vec<u8>,
    connection: &MysqlConnection
) -> QueryResult<usize>
{
    update(
        jobs::table
            .filter(jobs::user_id.eq(user_id))
            .filter(jobs::id.eq(job_id))
        )
        .set(jobs::options.eq(value))
        .execute(connection)
}