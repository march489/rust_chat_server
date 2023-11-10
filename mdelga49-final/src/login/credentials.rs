use crate::schema::*;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct LoginCredentials {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}
