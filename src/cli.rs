use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "new", alias = "n")]
    #[command(about = "Alias: n \nCreate a new Zettelkasten note.")]
    New {
        // #[arg(short, long)]
        title: String,
        #[arg(long)]
        r#type: String,
        #[arg(long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    // List {},
    // Edit {},
    // Archive {},
    // Remove {},
    // View {},
}

pub fn dispatch(cli: Cli) {
    match cli.command {
        Commands::New {
            title,
            r#type,
            tags,
        } => {
            println!("{}", title);
            println!("{}", r#type);
            println!("{:?}", tags);
        }
    }
}
