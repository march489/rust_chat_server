#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
extern crate diesel;

mod handler;
mod post;
mod schema;
mod tests;

use rocket::response::Redirect;
use rocket::Request;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/diesel", handler::list()))
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find {}. Please try again.", req.uri())
}

#[catch(500)]
fn db_error(req: &Request) -> String {
    format!("Something happened at the db level:\n{}", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found, db_error])
        .attach(handler::stage())
}
