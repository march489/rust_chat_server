// #[macro_use]
pub mod models;
pub mod schema;

extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// #[database("diesel")]
// struct Db(diesel::SqliteConnection);

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {database_url}"))
}

use self::models::{NewPost, Post};

pub fn create_post(conn: &mut SqliteConnection, new_title: &str, new_body: &str) {
    use crate::schema::posts;

    let new_post: NewPost = NewPost {
        title: new_title,
        body: new_body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    let results = posts::table
        .filter(posts::dsl::title.like(format!("%{}%", new_post.title)))
        .load::<Post>(conn)
        .expect("Error getting new post");

    for result in results {
        println!("{:?}", result);
    }
}
