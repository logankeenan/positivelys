
// use crate::repositories::positivelys_repository::create_positively_table;
use diesel::{SqliteConnection, Connection};

embed_migrations!("./migrations");

pub fn establish_connection(database_path: String) -> SqliteConnection {
        SqliteConnection::establish(&database_path).unwrap()
}

pub fn run_migrations(connection: &SqliteConnection) {
    embedded_migrations::run(connection);
}