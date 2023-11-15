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

#[get("/all_messages")]
async fn load_user_messages(db: Db) -> Option<Json<Vec<Message>>> {
    db.run(move |conn| {
        posts::table
            .inner_join(rooms::table)
            .inner_join(users::table)
            .select((rooms::room_name, users::display_name, posts::body))
            .load::<Message>(conn)
    })
    .await
    .map(Json)
    .ok()
}

#[get("/room/name/<room_name>")]
async fn get_room_id_by_name(db: Db, room_name: &str) -> Option<Json<Response>> {
    let name: String = String::from(room_name);
    let result: Option<Room> = db
        .run(move |conn| rooms::table.filter(rooms::room_name.eq(name)).first(conn))
        .await
        // .map(Json)
        .ok();

    let response: Response = match result {
        Some(room) => Response::new(true, Some(room.id.unwrap()), None).unwrap(),
        None => Response::new(
            false,
            None,
            Some(format!("Room {} does not exist.", &room_name)),
        )
        .unwrap(),
    };

    Some(Json(response))
}

#[get("/room/id/<id>")]
async fn get_messages_by_room_id(db: Db, id: i32) -> Option<Json<Response>> {
    let result: Option<Room> = db
        .run(move |conn| rooms::table.filter(rooms::id.eq(id)).first(conn))
        .await
        .ok();

    let response: Response = match result {
        Some(room) => Response::new(true, Some(room.id.unwrap()), None).unwrap(),
        None => Response::new(
            false,
            None,
            Some(format!("Room with id {} does not exist.", id)),
        )
        .unwrap(),
    };

    Some(Json(response))
}

#[post("/", data = "<post>")]
async fn create_post(db: Db, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
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

#[get("/<id>")]
async fn get_post_by_id(db: Db, id: i32) -> Option<Json<Post>> {
    println!("read_one(db, {id})");
    db.run(move |conn| posts::table.filter(posts::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[delete("/<id>")]
async fn delete_post(db: Db, id: i32) -> Result<Option<()>> {
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
async fn destroy_posts(db: Db) -> Result<()> {
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
                    create_post,
                    get_post_by_id,
                    delete_post,
                    destroy_posts,
                    load_user_messages,
                    create_room,
                    get_room_id_by_name,
                    get_messages_by_room_id
                ],
            )
    })
}
