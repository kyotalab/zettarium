use crate::{
    AppConfig, handler::zettel::zettel_new_handler, init_handler, zettel_archive_handler,
    zettel_edit_handler, zettel_find_handler, zettel_list_handler, zettel_remove_handler,
    zettel_view_handler,
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
    #[command(name = "init", about = "Initialize zettarium environment.")]
    Init,
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
    #[command(name = "edit", alias = "e")]
    #[command(about = "Alias: e \nOpen editor and Edit Zettelkasten note.")]
    Edit {
        id: String,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(long)]
        type_: Option<String>,
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
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
    #[command(name = "find", alias = "f")]
    #[command(about = "Alias: f \nFind interactively Zettelkasten note by keyword.")]
    Find {
        keyword: Option<String>,
        #[arg(long, help = "Match against title only", action = clap::ArgAction::SetTrue)]
        title_only: bool,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        link: bool,
    },
    // #[command(name = "backlink", alias = "bln")]
    // #[command(about = "Alias: f \nList backlinks.")]
    // BackLink {
    //     id: Option<String>,
    //     #[arg(long, action = clap::ArgAction::SetTrue)]
    //     out_going: bool,
    // },
}

pub fn dispatch(cli: Cli, conn: Option<&mut SqliteConnection>, config: &AppConfig) -> Result<()> {
    match cli.command {
        Commands::Init => {
            init_handler(config)?;
            Ok(())
        }
        Commands::New { title, type_, tags } => {
            let conn = conn.expect("DB connection not available");
            zettel_new_handler(conn, &title, &type_, &tags, config)?;
            Ok(())
        }
        Commands::List {
            id,
            type_,
            tags,
            all,
            archived,
        } => {
            let conn = conn.expect("DB connection not available");
            zettel_list_handler(conn, id.as_deref(), type_.as_deref(), &tags, all, archived)?;
            Ok(())
        }
        Commands::Edit {
            id,
            title,
            type_,
            tags,
        } => {
            let conn = conn.expect("DB connection not available");
            zettel_edit_handler(conn, &id, title.as_deref(), type_.as_deref(), &tags, config)?;
            Ok(())
        }
        Commands::Archive { id } => {
            let conn = conn.expect("DB connection not available");
            zettel_archive_handler(conn, &id, config)?;
            Ok(())
        }
        Commands::Remove { id, force } => {
            let conn = conn.expect("DB connection not available");
            let _result = zettel_remove_handler(conn, &id, force, config)?;
            Ok(())
        }
        Commands::View { id } => {
            let conn = conn.expect("DB connection not available");
            zettel_view_handler(conn, &id, config)?;
            Ok(())
        }
        Commands::Find {
            keyword,
            title_only,
            link,
        } => {
            let conn = conn.expect("DB connection not available");
            zettel_find_handler(conn, keyword.as_deref(), title_only, link, config)?;
            Ok(())
        }
    }
}
