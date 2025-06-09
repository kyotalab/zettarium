use crate::handler::zettel::zettel_new_handler;
use anyhow::Result;
use clap::{Parser, Subcommand};
use diesel::SqliteConnection;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "new", alias = "n")]
    #[command(about = "Alias: n \nCreate a new Zettelkasten note.")]
    New {
        // #[arg(short, long)]
        title: String,
        #[arg(long)]
        type_: String,
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    // List {},
    // Edit {},
    // Archive {},
    // Remove {},
    // View {},
}

pub fn dispatch(cli: Cli, conn: &mut SqliteConnection) -> Result<()> {
    match cli.command {
        Commands::New { title, type_, tags } => {
            zettel_new_handler(conn, &title, &type_, &tags)?;
            Ok(())
        }
    }
}
