use super::schema::test;

use serde::Serialize;

#[derive(Queryable, PartialEq, Serialize)]
pub struct Test {
    pub id: i32,
    pub t_key: String,
    pub t_value: String,
}

#[derive(Insertable)]
#[table_name="test"]
pub struct NewTest<'a> {
    pub t_key: &'a str,
    pub t_value: &'a str,
}