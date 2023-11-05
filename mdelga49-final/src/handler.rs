use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{Build, Rocket};

use crate::post::Post;
use crate::schema::*;

#[database("diesel")]
struct Db(diesel::SqliteConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/", data = "<post>")]
async fn create(db: Db, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
    println!("Hi, you're in POST, and here is your data:\n{:?}", post);
    let post_value = post.clone();
    let id: Option<i32> = db
        .run(move |conn| {
            diesel::insert_into(posts::table)
                .values(&*post_value)
                .returning(posts::id)
                .get_result(conn)
        })
        .await?;

    println!("We've come back from the database with the id [{:?}]", id);
    post.id = Some(id.expect("returning guarantees ID present"));
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
                routes![list, create, read_one, delete_one, destroy],
            )
    })
}
