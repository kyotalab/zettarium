use crate::AppConfig;
use anyhow::Result;
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection(config: &AppConfig) -> SqliteConnection {
    let database_url = &config.paths.db_path;
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations(conn: &mut SqliteConnection) -> Result<()> {
    let _ = conn.run_pending_migrations(MIGRATIONS);
    Ok(())
}
