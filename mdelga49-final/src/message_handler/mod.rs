use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types;
use rocket::fairing::AdHoc;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

pub mod message;
mod post;
mod room;
#[cfg(test)]
mod tests;

use crate::db::Db;
use crate::login::response::Response;
use crate::message_handler::message::Message;
use crate::message_handler::post::Post;
use crate::message_handler::room::Room;
use crate::schema::*;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
const LAST_INSERT_ROW_QUERY: &str = "SELECT last_insert_rowid()";
const ROOM_ALREADY_EXISTS: &str = "A room with that name already exists";

#[post("/room", data = "<room>")]
async fn create_room(db: Db, room: Json<Room>) -> Result<Created<Json<Option<Response>>>> {
    let new_room: Option<i32> = db
        .run(move |conn| {
            let room_value = room.clone();

            diesel::insert_into(rooms::table)
                .values(&*room_value)
                .execute(conn)
                .and_then(|_| {
                    sql::<sql_types::Integer>(LAST_INSERT_ROW_QUERY).get_result::<i32>(conn)
                })
        })
        .await
        .ok();

    let response: Response = match new_room {
        Some(id) => Response::new(true, Some(id), None).unwrap(),
        None => Response::new(false, None, Some(String::from(ROOM_ALREADY_EXISTS))).unwrap(),
    };

    Ok(Created::new("/").body(Json(Some(response))))
}

#[get("/all")]
async fn load_messages(db: Db) -> Option<Json<Vec<Message>>> {
    db.run(move |conn| {
        posts::table
            .select((posts::thread, posts::author, posts::body))
            .load::<Message>(conn)
    })
    .await
    .map(Json)
    .ok()
}

#[post("/", data = "<post>")]
async fn create(db: Db, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
    let post_value = post.clone();

    let inserted: Option<i32> = db
        .run(move |conn| {
            let result = diesel::insert_into(posts::table)
                .values(&*post_value)
                .execute(conn)
                .and_then(|_| {
                    sql::<sql_types::Integer>("SELECT last_insert_rowid()").get_result::<i32>(conn)
                });
            result.ok()
        })
        .await;

    post.id = inserted;
    Ok(Created::new("/").body(post))
}

#[get("/")]
async fn list(db: Db) -> Result<Json<Vec<Option<i32>>>> {
    let ids: Vec<Option<i32>> = db
        .run(move |conn| posts::table.select(posts::id).load(conn))
        .await?;

    Ok(Json(ids))
}

#[get("/<id>")]
async fn read_one(db: Db, id: i32) -> Option<Json<Post>> {
    println!("read_one(db, {id})");
    db.run(move |conn| posts::table.filter(posts::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[delete("/<id>")]
async fn delete_one(db: Db, id: i32) -> Result<Option<()>> {
    let affected: usize = db
        .run(move |conn| {
            diesel::delete(posts::table)
                .filter(posts::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}

#[delete("/")]
async fn destroy(db: Db) -> Result<()> {
    db.run(move |conn| diesel::delete(posts::table).execute(conn))
        .await?;
    Ok(())
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    Db::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount(
                "/diesel",
                routes![
                    list,
                    create,
                    read_one,
                    delete_one,
                    destroy,
                    load_messages,
                    create_room
                ],
            )
    })
}
