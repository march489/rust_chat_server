use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::schema::*;

#[database("diesel")]
struct Db(diesel::SqliteConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
struct Post {
    id: i32,
    author: String,
    thread: String,
    body: String,
    timestamp: i32,
}

#[post("/", data = "<post>")]
async fn create(db: Db, mut post: Json<Post>) -> Result<Created<Json<Post>>> {
    let post_value = post.clone();
    let id: i32 = db
        .run(move |conn| {
            diesel::insert_into(posts::table)
                .values(&*post_value)
                .returning(posts::id)
                .get_result(conn)
        })
        .await?;

    post.id = id;
    Ok(Created::new("/").body(post))
}

#[get("/")]
async fn list(db: Db) -> Result<Json<Vec<i32>>> {
    let ids: Vec<i32> = db
        .run(move |conn| posts::table.select(posts::id).load(conn))
        .await?;

    Ok(Json(ids))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .mount("/diesel_sqlite", routes![list, create])
    })
}
