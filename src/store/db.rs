use crate::AppConfig;
use diesel::prelude::*;

pub fn establish_connection(config: &AppConfig) -> SqliteConnection {
    let database_url = &config.paths.db_path;
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
