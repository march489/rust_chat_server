#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate diesel;
mod schema;

mod diesel_sqlite;

use rocket::response::Redirect;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/", diesel_sqlite::list()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/diesel_sqlite", routes![index])
        .attach(diesel_sqlite::stage())
}
