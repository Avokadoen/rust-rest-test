extern crate rust_rest;
extern crate diesel;

use self::rust_rest::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use rust_rest::schema::test::dsl::*;

    let connection = establish_connection();
    let results= test.filter(t_key.eq("hello"))
        .limit(5)
        .load::<Test>(&connection)
        .expect("Error loading test");

    println!("Displaying {} test data", results.len());
    for result in results {
        println!("id: {}", result.id);
        println!("key: {}", result.t_key);
        println!("value: {}", result.t_value);
        println!("----------\n");
    }
}