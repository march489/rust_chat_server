use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Queryable, Selectable, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: i32,
}

#[derive(Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
}

#[derive(Clone, Insertable, Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
