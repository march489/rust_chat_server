#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
// #[macro_use]
// extern crate rocket_sync_db_pools;

mod db;
mod login;
mod message_handler;
mod schema;

use crate::message_handler::message::Message;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, error::RecvError, Sender};
use rocket::{Request, Shutdown, State};

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find {}. Please try again.", req.uri())
}

#[catch(500)]
fn db_error(req: &Request) -> String {
    format!("Something happened at the db level:\n{}", req.uri())
}

// #[get("/events")]
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },

                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[post("/message", data = "<form>")]
fn post_message(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .register("/", catchers![not_found, db_error])
        .mount("/", routes![post_message, events])
        .mount("/", FileServer::from(relative!("static")))
        .attach(message_handler::stage())
        .attach(login::stage())
}
