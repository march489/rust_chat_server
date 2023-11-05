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

    /*  The code below would be ideal, but returning returns the PREVIOUS value of id,
    which for our posts would be NONE, and this causes a panic */
    // let inserted: Option<i32> = db
    //     .run(move |conn| {
    //         diesel::insert_into(posts::table)
    //             .values(&*post_value)
    //             .returning(posts::id)
    //             .get_result(conn)
    //     })
    //     .await?;

    /*  This code is less ideal since it essentially requires us to do a matching
    query after insertion to get the ID number attached to it back.
    TODO: Maybe attach our own UUID to track on our end? */
    let inserted: usize = db
        .run(move |conn| {
            diesel::insert_into(posts::table)
                .values(&*post_value)
                .execute(conn)
        })
        .await?;

    assert_eq!(inserted, 1);

    let post_val2 = post.clone();

    let result: Option<Json<Post>> = db
        .run(move |conn| {
            posts::table
                .filter(posts::body.eq(&post_val2.body))
                .first(conn)
        })
        .await
        .map(Json)
        .ok();

    println!("And here is the POST result:\n{:?}", result);
    post.id = result.unwrap().id;
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
