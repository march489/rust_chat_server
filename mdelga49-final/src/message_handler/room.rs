use crate::schema::*;
use diesel::prelude::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: Option<i32>,
    pub room_name: String,
}

impl Room {
    pub fn new(name: &str) -> Room {
        Room {
            id: None,
            room_name: String::from(name),
        }
    }
}
