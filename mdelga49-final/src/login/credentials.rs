use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct LoginCredentials {
    pub username: String,
    pub hashed_pasword: String,
}
