use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types;
use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::response::Debug;
use rocket::serde::json::Json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub mod credentials;
pub mod response;

use crate::db::Db;
use crate::login::credentials::LoginCredentials;
use crate::schema::*;
use response::Response;
#[cfg(test)]
mod test;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
const NO_MATCH_ERR_MSG: &str = "Invalid username or password";

#[post("/auth", data = "<credentials>")]
async fn authorize_user(
    db: Db,
    credentials: Json<LoginCredentials>,
) -> Result<Json<Option<Response>>> {
    let creds: LoginCredentials = credentials.into_inner();
    let entered_username: String = creds.username.clone();
    let entered_password: String = creds.password.clone();

    let returned_credentials: Option<LoginCredentials> = db
        .run(move |conn| {
            users::table
                .filter(users::username.eq(entered_username))
                .first(conn)
        })
        .await
        .ok();

    match returned_credentials {
        Some(credentials) => {
            let returned_password: &String = &credentials.password;
            let mut hasher = DefaultHasher::new();

            entered_password.hash(&mut hasher);
            if hasher.finish().to_string().eq(returned_password) {
                Ok(Json(Response::new(true, None)))
            } else {
                Ok(Json(Response::new(
                    false,
                    Some(NO_MATCH_ERR_MSG.to_string()),
                )))
            }
        }
        None => Ok(Json(Response::new(
            false,
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
async fn query_user_by_id(db: Db, id: i32) -> Option<Json<LoginCredentials>> {
    db.run(move |conn| users::table.filter(users::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[get("/email/<email_username>")]
async fn query_user_by_email(db: Db, email_username: String) -> Option<Json<LoginCredentials>> {
    db.run(move |conn| {
        users::table
            .filter(users::username.eq(email_username))
            .first(conn)
    })
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

#[post("/", data = "<credentials>")]
async fn create(db: Db, credentials: Json<LoginCredentials>) -> Result<Created<Json<Option<i32>>>> {
    let new_user_credentials: Json<LoginCredentials> = credentials.clone();

    let new_user_id: Option<i32> = db
        .run(move |conn| {
            let result = diesel::insert_into(users::table)
                .values(&*new_user_credentials)
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
