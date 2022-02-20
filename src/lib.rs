#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod taskdb;

use diesel::prelude::*;

pub fn create_connection(db_url: String) -> SqliteConnection {
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| panic!("fail to open sqlite db: {}", db_url))
}
