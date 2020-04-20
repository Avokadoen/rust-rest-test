extern crate rust_rest;
extern crate diesel;

use self::diesel::prelude::*;
use self::rust_rest::*;
use self::models::Test;
use std::env::args;

fn main() {
    use rust_rest::schema::test::dsl::{test, t_value};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(test.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}