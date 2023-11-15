use crate::schema::*;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub password: String,
    pub display_name: String,
}

impl User {
    pub fn new(email: &String, password: &String, name: &String) -> User {
        User {
            id: None,
            email: email.clone(),
            password: password.clone(),
            display_name: name.clone(),
        }
    }
}
