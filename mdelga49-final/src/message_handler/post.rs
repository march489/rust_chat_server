use crate::schema::*;
use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Option<i32>,
    pub user_id: i32,
    pub room_id: i32,
    pub body: String,
    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
}

impl Post {
    pub fn new(author_id: i32, thread_id: i32, bod: &str) -> Post {
        Post {
            id: None,
            user_id: author_id,
            room_id: thread_id,
            body: String::from(bod),
            created_at: None,
        }
    }
}
