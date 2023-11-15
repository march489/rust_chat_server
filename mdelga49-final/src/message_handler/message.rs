use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromForm, Serialize, Deserialize, Queryable)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

// The problem is that the Message struct doesn't actually give us the info we need
