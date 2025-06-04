use anyhow::Result;
use clap::Parser;
use zettarium::{Cli, cli};

// for test
use chrono::Local;
use zettarium::model::{Tag, Zettel, parse_note_type};

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli::dispatch(cli);

    let zettel = Zettel {
        id: "yyyymmddhhmmss".into(),
        title: "this is a test".into(),
        r#type: parse_note_type("fleeting")?,
        created: Local::now().naive_local(),
        updated: Local::now().naive_local(),
        archived: false,
    };

    println!("{:?}", zettel);

    let tags_str = vec!["rust", "test"];
    for t in tags_str {
        let tag = Tag {
            id: "t-001".into(),
            tag_name: t.into(),
        };
        println!("{:?}", tag);
    }

    Ok(())
}
