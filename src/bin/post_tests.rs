extern crate rust_rest;
extern crate diesel;

use self::rust_rest::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your key to be?");
    let mut key = String::new();
    stdin().read_line(&mut key).unwrap();
    let key = &key[..(key.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", key, EOF);
    let mut value = String::new();
    stdin().read_to_string(&mut value).unwrap();

    let test = create_test(&connection, key, &value);
    match test {
        Ok(result) => println!("\nSaved draft {} with id {}", key, result.id),
        Err(e) => println!("{}", e)
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";