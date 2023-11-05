use crate::schema::*;
// use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct Post {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub author: String,
    pub thread: String,
    pub body: String,
    // #[serde(skip_deserializing)]
    // pub created_at: Option<NaiveDateTime>,
}

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct TestPost {
    pub author: String,
    pub thread: String,
    pub body: String,
    // #[serde(skip_deserializing)]
    // pub created_at: Option<NaiveDateTime>,
}

impl TestPost {
    pub fn new(aut: &str, thr: &str, bod: &str) -> TestPost {
        TestPost {
            author: String::from(aut),
            thread: String::from(thr),
            body: String::from(bod),
            // created_at: None,
        }
    }
}
