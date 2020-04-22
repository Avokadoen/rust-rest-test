#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate rust_rest;
extern crate diesel;

use self::rust_rest::*;
use self::rust_rest::models::Test;

use rocket_contrib::json::Json;

#[get("/by_id/<id>")]
fn get_by_id<'a>(id: i32) -> Json<Test> {
    let connection = establish_connection();

    let found = find(&connection, id).unwrap();

    Json(found)
}

fn main() {
    rocket::ignite().mount("/", routes![get_by_id]).launch();
}
