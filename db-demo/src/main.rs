#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::diesel;

// mod diesel_sqlite;

use diesel::prelude::*;

#[database("diesel")]
struct Db(diesel::SqliteConnection);

fn main() {
    rocket::ignite().attach(Db::fairing()).launch();
}
