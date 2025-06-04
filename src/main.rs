use anyhow::Result;
use clap::Parser;
use zettarium::{Cli, cli};

// for test
use chrono::Local;
use zettarium::model::{Body, FrontMatter, Markdown, NoteType, Tag, Zettel};
use zettarium::validate::dedup_and_warn;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli::dispatch(cli);

    let zettel = Zettel {
        id: "yyyymmddhhmmss".into(),
        title: "this is a test".into(),
        r#type: "fleeting".parse::<NoteType>()?,
        created: Local::now().naive_local(),
        updated: Local::now().naive_local(),
        archived: false,
    };

    println!("{zettel}");

    let tags_str = vec!["rust", "Rust", "test"]
        .into_iter()
        .map(String::from)
        .collect();
    let cleaned_tags = dedup_and_warn(tags_str);
    // for (i, t) in cleaned_tags.iter().enumerate() {
    //     let tag = Tag {
    //         id: format!("t-{:03}", i + 1),
    //         tag_name: t.to_string(),
    //     };
    //     println!("{tag}");
    // }

    let front_matter = FrontMatter {
        zettel,
        tags: cleaned_tags,
    };

    let body = Body(format!("## {}", front_matter.get_zettel().title));

    let markdown = Markdown { front_matter, body };

    println!("{markdown}");

    Ok(())
}
