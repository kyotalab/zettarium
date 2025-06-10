use crate::{
    handler::zettel::zettel_new_handler, zettel_archive_handler, zettel_list_handler,
    zettel_remove_handler, zettel_view_handler,
};
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
    #[command(name = "list", alias = "ls")]
    #[command(about = "Alias: ls \nList Zettelkasten notes.")]
    List {
        #[arg(long)]
        id: Option<String>,
        #[arg(long)]
        type_: Option<String>,
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        all: bool,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        archived: bool,
    },
    // Edit {},
    #[command(name = "archive", alias = "arc")]
    #[command(about = "Alias: arc \nArchive Zettelkasten note.")]
    Archive { id: String },
    #[command(name = "remove", alias = "rm")]
    #[command(about = "Alias: rm \nDelete Zettelkasten note.")]
    Remove {
        id: String,
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        force: bool,
    },
    #[command(name = "view", alias = "v")]
    #[command(about = "Alias: v \nView Zettelkasten note in detail.")]
    View { id: String },
}

pub fn dispatch(cli: Cli, conn: &mut SqliteConnection) -> Result<()> {
    match cli.command {
        Commands::New { title, type_, tags } => {
            zettel_new_handler(conn, &title, &type_, &tags)?;
            Ok(())
        }
        Commands::List {
            id,
            type_,
            tags,
            all,
            archived,
        } => {
            zettel_list_handler(conn, id.as_deref(), type_.as_deref(), &tags, all, archived)?;
            Ok(())
        }
        Commands::Archive { id } => {
            zettel_archive_handler(conn, &id)?;
            Ok(())
        }
        Commands::Remove { id, force } => {
            let _result = zettel_remove_handler(conn, &id, force)?;
            Ok(())
        }
        Commands::View { id } => {
            zettel_view_handler(conn, &id)?;
            Ok(())
        }
    }
}
