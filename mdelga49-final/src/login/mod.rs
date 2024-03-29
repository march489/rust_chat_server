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
const LAST_INSERT_ROW_QUERY: &str = "SELECT last_insert_rowid()";

#[post("/shibboleth", data = "<credentials>")]
async fn authorize_user(db: Db, credentials: Json<User>) -> Result<Json<Option<Response>>> {
    let creds: User = credentials.into_inner();
    let entered_email: String = creds.email.clone();
    let entered_password: String = creds.password.clone();

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
            if hashed_password.eq(returned_password) {
                Ok(Json(Response::new(
                    true,
                    Some(credentials.id.unwrap()),
                    Some(credentials.display_name),
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
async fn list_users_ids(db: Db) -> Result<Json<Vec<Option<i32>>>> {
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
async fn query_user_by_email(db: Db, email: &str) -> Option<Json<User>> {
    let address = String::from(email);
    db.run(move |conn| users::table.filter(users::email.eq(address)).first(conn))
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
async fn create_user(db: Db, user: Json<User>) -> Result<Created<Json<Option<Response>>>> {
    let mut new_user: Json<User> = user.clone();

    let mut hasher = DefaultHasher::new();
    new_user.password.hash(&mut hasher);
    new_user.password = hasher.finish().to_string();

    let result = db
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(&*new_user)
                .execute(conn)
        })
        .await;

    let response = match result {
        Ok(_) => {
            let new_id_number: i32 = db
                .run(move |conn| {
                    sql::<sql_types::Integer>(LAST_INSERT_ROW_QUERY)
                        .get_result::<i32>(conn)
                        .ok()
                        .unwrap()
                })
                .await;
            Ok(Created::new("/").body(Json(Response::new(true, Some(new_id_number), None))))
        }
        Err(_) => Ok(Created::new("/").body(Json(Response::new(
            false,
            None,
            Some(String::from(
                "The entered email address already has an account.",
            )),
        )))),
    };

    response
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
                create_user,
                list_users_ids,
                authorize_user
            ],
        )
    })
}
