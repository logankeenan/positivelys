use rusqlite::{params, Connection};
use crate::repositories::positivelys_repository::create_positively_table;
use routines::models::app_request::AppContext;

pub fn create_database(database_path: String) -> Connection {
    let connection = Connection::open(database_path).unwrap();

    connection
}

pub fn run_migrations(connection: &Connection) {
    create_positively_table(connection);
}