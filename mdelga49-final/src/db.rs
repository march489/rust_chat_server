// use diesel::prelude::*;
use rocket_sync_db_pools::database;
// use rocket_sync_db_pools::diesel::r2d2::ConnectionManager;

#[database("diesel")]
pub struct Db(diesel::SqliteConnection);
