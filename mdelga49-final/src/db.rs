use rocket_sync_db_pools::database;

#[database("diesel")]
pub struct Db(diesel::SqliteConnection);
