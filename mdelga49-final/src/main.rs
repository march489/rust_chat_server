#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate diesel;
mod schema;

mod diesel_sqlite;

use rocket::response::Redirect;
use rocket::Request;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/diesel", diesel_sqlite::list()))
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find {}. Please try again.", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .attach(diesel_sqlite::stage())
}
