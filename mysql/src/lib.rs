//! this module contains the database schema  
//! the struct representations of the tables  
//! the simple queries for single tables
//! and more complex transactions below
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

pub mod schema;

pub mod admin;
pub mod jobs;
pub mod journal;
pub mod printers;
pub mod user;

use diesel::{
    prelude::*,
    result::Error,
    mysql::MysqlConnection,
    r2d2::{
        ConnectionManager,
        Pool,
    },
};

use std::env;

/// all the transactions are found here
use journal::insert::insert_into_journal;
use user::update::update_user_credit;

/// import a credit for an existing user
pub fn import_credit(user_id: u32, credit: i32, connection: &MysqlConnection) -> QueryResult<()>
{
    connection.transaction::<_, Error, _>(|| {
        let _affected_rows = insert_into_journal(user_id, credit, credit, None, None, "import", connection)?;

        let _affected_rows = update_user_credit(user_id, credit, connection)?;

        Ok(())
    })
}

use user::{
    insert::{
        UserInsert,
        insert_into_user,
    },
    select::select_user_id_by_name,
};

/// insert a new user
pub fn insert_user(name: &str, hash: Vec<u8>, salt: Vec<u8>, email: Option<String>, locked: bool, connection: &MysqlConnection) -> QueryResult<u32>
{
    connection.transaction::<_, Error, _>(|| {
        let _affected_rows = insert_into_user(UserInsert{
            name, hash, salt, email, card: None, pin: None, locked}, connection)?;

        assert_eq!(_affected_rows, 1);

        let user_id = select_user_id_by_name(name, connection)?;

        let _affected_rows = insert_into_journal(user_id, 0, 0, None, None, "created", connection)?;

        Ok(user_id)
    })
}

use journal::select::select_latest_credit_of_user;

/// update the credit of an user
pub fn update_credit_as_admin(user_id: u32, value: i32, admin_id: u32, description: &str, connection: &MysqlConnection) -> QueryResult<i32>
{
    connection.transaction::<_, Error, _>(|| {
        let credit = value + select_latest_credit_of_user(user_id, connection)?;

        let _affected_rows = insert_into_journal(user_id, credit, value, None, Some(admin_id), description, connection)?;

        let _affected_rows = update_user_credit(user_id, credit, connection)?;

        Ok(credit)
    })
}


use journal::select::{
    select_latest_print_journal_id,
    select_journal_token_by_id,
};
use journal::update::update_journal_token;

/// update the credit with a journal token
pub fn update_credit_with_unused_token(user_id: u32, token_id: u32, connection: &MysqlConnection) -> QueryResult<i32>
{
    connection.transaction::<_, Error, _>(|| {
        let token = select_journal_token_by_id(token_id, connection)?;
        assert!(!token.used);
        let value = token.value as i32;

        let credit = value + select_latest_credit_of_user(user_id, connection)?;

        let _affected_rows = insert_into_journal(user_id, credit, value, None, None, &format!("created with token {}", token.content), connection)?;

        let _affected_rows = update_journal_token(token.id, true, user_id, connection)?;

        let _affected_rows = update_user_credit(user_id, credit, connection)?;

        Ok(credit)
    })
}

use journal::insert::insert_into_print_journal;

#[derive(Debug, Clone)]
pub struct CreditUpdate
{
    pub user_id: u32,
    pub value: i32,
    pub job_id: u32,
    pub pages: u16,
    pub colored: u16,
    pub score: i16,
    pub device_id: u32,
    pub options: Vec<u8>,
}

/// update the credit and journal after printing with extra information in the print journal
pub fn update_credit_after_print(update: CreditUpdate, connection: &MysqlConnection) -> QueryResult<i32>
{
    connection.transaction::<_, Error, _>(|| {
        let _rows_affected = insert_into_print_journal(update.job_id, update.pages, update.colored, update.score, update.device_id, update.options, connection)?;

        let print_id = select_latest_print_journal_id(connection)?;

        let credit = update.value + select_latest_credit_of_user(update.user_id, connection)?;

        let _rows_affected = insert_into_journal(update.user_id, credit, update.value, Some(print_id), None, &format!("{} Seiten", update.pages), connection)?;

        let _rows_affected = update_user_credit(update.user_id, credit, connection)?;

        Ok(credit)
    })
}

use journal::{
    select::{
        select_journal_of_user_with_limit_and_offset,
        select_journal_with_limit_and_offset,
        select_print_journal_by_id,
    },
    PrintJournal,
    Journal,
};

/// select full journal of a user including print journal
pub fn select_full_journal_of_user(user_id: u32, limit: i64, offset: i64, connection: &MysqlConnection) -> QueryResult<Vec<(Journal, Option<PrintJournal>)>>
{
    connection.transaction::<_, Error, _>(|| {
        let journal = select_journal_of_user_with_limit_and_offset(user_id, limit, offset, connection)?;
        let mut full_journal: Vec<(Journal, Option<PrintJournal>)> = Vec::with_capacity(journal.len());

        for entry in journal {
            if let Some(print_id) = entry.print_id {
                full_journal.push((entry, Some(select_print_journal_by_id(print_id, connection)?)));
            } else {
                full_journal.push((entry, None));
            }
        }
        Ok(full_journal)
    })
}

/// select the full journal including print journal
pub fn select_full_journal(limit: i64, offset: i64, connection: &MysqlConnection) -> QueryResult<Vec<(Journal, Option<PrintJournal>)>>
{
    connection.transaction::<_, Error, _>(|| {
        let journal = select_journal_with_limit_and_offset(limit, offset, connection)?;
        let mut full_journal: Vec<(Journal, Option<PrintJournal>)> = Vec::with_capacity(journal.len());

        for entry in journal {
            if let Some(print_id) = entry.print_id {
                full_journal.push((entry, Some(select_print_journal_by_id(print_id, connection)?)));
            } else {
                full_journal.push((entry, None));
            }
        }
        Ok(full_journal)
    })
}


/// wrapper for pool builder
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

/// get the astaprint mysql pool, pass the number of connections
pub fn get_mysql_pool(
    max_size: u32,
) -> Pool<ConnectionManager<MysqlConnection>>
{
    create_mysql_pool(
        &env::var("ASTAPRINT_DATABASE_URL")
            .expect("reading database url from environment"),
        max_size,
    )
}

#[cfg(test)]
mod tests
{
    use crate::{
        create_mysql_pool,
        admin::select::*,
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
        let print_journal = select_print_journal(connection).unwrap();
        println!("{:x?}", print_journal);
        let journal_tokens = select_journal_tokens(connection).unwrap();
        println!("{:x?}", journal_tokens);

        let printers = select_printers(connection).unwrap();
        println!("{:x?}", printers);

        let user = select_user(connection).unwrap();
        println!("{:x?}", user);
        let user_tokens = select_user_tokens(connection).unwrap();
        println!("{:x?}", user_tokens);

        //let jobs = select_jobs(connection).unwrap();
        //println!("{:x?}", jobs);

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

    fn test_select_print_journal()
    {
        let url = env::var("ASTAPRINT_DATABASE_URL").expect("getting url from env");
        let pool = create_mysql_pool(&url, 3);
        let connection = &pool.get().unwrap();
    }
}
