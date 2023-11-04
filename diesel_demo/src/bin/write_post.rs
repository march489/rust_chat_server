extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();
}
