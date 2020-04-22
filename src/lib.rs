#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::result::Error;

use dotenv::dotenv;
use std::env;

use self::models::{NewTest, Test};

pub mod schema;
pub mod models;

use schema::test::dsl::*;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    
    MysqlConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_test<'a>(conn: &MysqlConnection, key: &'a str, value: &'a str) -> Result<Test, diesel::result::Error> {
    conn.transaction::<Test, Error, _>(|| {
        let new_test = NewTest {
            t_key: key,
            t_value: value,
        };

        let inserted_count = diesel::insert_into(test)
            .values(&new_test)
            .execute(conn)?; // TODO: never unwrap in production

        // TODO: Workaround as diesel does not support getting latest inserted object in sql (only postgresql).
        //       This is going to break in production
        Ok(test
            .order(id.desc())
            .limit(inserted_count as i64)
            .load::<Test>(conn)?
            .into_iter()
            .rev()
            .nth(0)
            .unwrap())  // TODO: never unwrap in production
    })
}

pub fn find(conn: &MysqlConnection, q_id: i32) -> Result<Test, Error> {
    test.find(q_id).first::<Test>(conn)
}