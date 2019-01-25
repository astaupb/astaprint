#[macro_use]
extern crate diesel;
pub mod schema;

pub mod admin;
pub mod jobs;
pub mod journal;
pub mod printers;
pub mod user;

use diesel::{
    mysql::MysqlConnection,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use std::env;

pub fn create_mysql_pool(
    url: &str,
    max_size: u32,
) -> Pool<ConnectionManager<MysqlConnection>>
{
    Pool::builder()
        .max_size(max_size)
        .build(ConnectionManager::<MysqlConnection>::new(url))
        .expect("creating Mysql Connection Pool")
}

pub fn get_pool(
) -> Pool<ConnectionManager<MysqlConnection>>
{
    create_mysql_pool(
        &env::var("ASTAPRINT_DATABASE_URL")
            .expect("reading database url from environment"),
        1,
    )
}
#[cfg(test)]
mod tests
{
    use crate::{
        admin::select::*,
        create_mysql_pool,
        jobs::select::*,
        journal::select::*,
        printers::select::*,
        user::select::*,
    };
    use diesel::prelude::*;
    use std::env;
    fn select_everything(connection: &MysqlConnection)
    {
        let journal = select_journal(connection).unwrap();
        println!("{:x?}", journal);
        let journal_digest = select_journal_digest(connection).unwrap();
        println!("{:x?}", journal_digest);
        let journal_token = select_journal_token(connection).unwrap();
        println!("{:x?}", journal_token);

        let printers = select_printers(connection).unwrap();
        println!("{:x?}", printers);
        let printer_counter = select_printer_counter(connection).unwrap();
        println!("{:x?}", printer_counter);

        let user = select_user(connection).unwrap();
        println!("{:x?}", user);
        let user_tokens = select_user_tokens(connection).unwrap();
        println!("{:x?}", user_tokens);

        let jobs = select_jobs(connection).unwrap();
        println!("{:x?}", jobs);

        let admin = select_admin(connection).unwrap();
        println!("{:x?}|", admin);
        let admin_tokens = select_admin_tokens(connection).unwrap();
        println!("{:x?}", admin_tokens);
    }
    #[test]
    fn test_select_everything()
    {
        let url = env::var("ASTAPRINT_DATABASE_URL").expect("getting url from env");
        let pool = create_mysql_pool(&url, 3);
        let connection = &pool.get().unwrap();
        select_everything(connection);
    }
}
