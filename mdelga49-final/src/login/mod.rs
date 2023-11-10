use diesel::prelude::*;
use rocket::fairing::AdHoc;
use rocket::response::Debug;
use rocket::serde::json::Json;

pub mod credentials;

use crate::schema::*;

#[database("diesel")]
pub struct Db(diesel::SqliteConnection);

// use crate::login::credentials::LoginCredentials;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/")]
async fn list_users(db: Db) -> Result<Json<Vec<Option<i32>>>> {
    let ids: Vec<Option<i32>> = db
        .run(move |conn| users::table.select(users::id).load(conn))
        .await?;

    Ok(Json(ids))
}

#[get("/auth/<email>/<password>")]
async fn authenticate_user(_db: Db, email: String, password: String) {
    println!("querying email {email} with password {password}");
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Login Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .mount("/login", routes![authenticate_user, list_users])
    })
}
