use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types;
use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::response::Debug;
use rocket::serde::json::Json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub mod response;
pub mod user;

use crate::db::Db;
use crate::login::user::User;
use crate::schema::*;
use response::Response;
#[cfg(test)]
mod test;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
const NO_MATCH_ERR_MSG: &str = "Invalid username or password";

#[post("/shibboleth", data = "<credentials>")]
async fn authorize_user(db: Db, credentials: Json<User>) -> Result<Json<Option<Response>>> {
    let creds: User = credentials.into_inner();
    let entered_email: String = creds.email.clone();
    let entered_password: String = creds.password.clone();

    // DEBUGGING
    println!("entered password: {entered_password}");

    let returned_credentials: Option<User> = db
        .run(move |conn| {
            users::table
                .filter(users::email.eq(entered_email))
                .first(conn)
        })
        .await
        .ok();

    match returned_credentials {
        Some(credentials) => {
            let returned_password: &String = &credentials.password;
            let mut hasher = DefaultHasher::new();

            entered_password.hash(&mut hasher);
            let hashed_password: String = hasher.finish().to_string();
            // DEBUGGING
            println!("hashed password: {hashed_password}");
            println!("returned password: {returned_password}");
            if hashed_password.eq(returned_password) {
                Ok(Json(Response::new(
                    true,
                    Some(credentials.id.unwrap()),
                    None,
                )))
            } else {
                Ok(Json(Response::new(
                    false,
                    None,
                    Some(NO_MATCH_ERR_MSG.to_string()),
                )))
            }
        }
        None => Ok(Json(Response::new(
            false,
            None,
            Some(NO_MATCH_ERR_MSG.to_string()),
        ))),
    }
}

#[get("/")]
async fn list_users(db: Db) -> Result<Json<Vec<Option<i32>>>> {
    let ids: Vec<Option<i32>> = db
        .run(move |conn| {
            users::table
                .select(users::id)
                .order(users::id.asc())
                .load(conn)
        })
        .await?;

    Ok(Json(ids))
}

#[get("/id/<id>")]
async fn query_user_by_id(db: Db, id: i32) -> Option<Json<User>> {
    db.run(move |conn| users::table.filter(users::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[get("/email/<email>")]
async fn query_user_by_email(db: Db, email: String) -> Option<Json<User>> {
    db.run(move |conn| users::table.filter(users::email.eq(email)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[delete("/<id>")]
async fn delete_user(db: Db, id: i32) -> Result<Option<()>> {
    let affected: usize = db
        .run(move |conn| {
            diesel::delete(users::table)
                .filter(users::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}

#[delete("/")]
async fn destroy(db: Db) -> Result<()> {
    db.run(move |conn| diesel::delete(users::table).execute(conn))
        .await?;
    Ok(())
}

#[post("/", data = "<user>")]
async fn create(db: Db, user: Json<User>) -> Result<Created<Json<Option<i32>>>> {
    let mut new_user: Json<User> = user.clone();

    let mut hasher = DefaultHasher::new();
    new_user.password.hash(&mut hasher);
    new_user.password = hasher.finish().to_string();

    let new_user_id: Option<i32> = db
        .run(move |conn| {
            let result = diesel::insert_into(users::table)
                .values(&*new_user)
                .execute(conn)
                .and_then(|_| {
                    sql::<sql_types::Integer>("SELECT last_insert_rowid()").get_result::<i32>(conn)
                });
            result.ok()
        })
        .await;

    Ok(Created::new("/").body(Json(new_user_id)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Login Stage", |rocket| async {
        rocket.mount(
            "/auth",
            routes![
                query_user_by_email,
                query_user_by_id,
                destroy,
                delete_user,
                create,
                list_users,
                authorize_user
            ],
        )
    })
}
