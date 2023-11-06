// use crate::schema::*;
// use chrono::naive::NaiveDateTime;
// use diesel::prelude::*;
// use rocket::serde::{Deserialize, Serialize};

// #[derive(PartialEq, Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
// #[serde(crate = "rocket::serde")]
// #[diesel(table_name = posts)]
// pub struct Post {
//     pub id: Option<i32>,
//     pub author: String,
//     pub thread: String,
//     pub body: String,
//     #[serde(skip_deserializing)]
//     pub created_at: Option<NaiveDateTime>,
// }

// impl Post {
//     pub fn new(aut: &str, thr: &str, bod: &str) -> Post {
//         Post {
//             id: None,
//             author: String::from(aut),
//             thread: String::from(thr),
//             body: String::from(bod),
//             created_at: None,
//         }
//     }
// }
