use crate::schema::*;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}
