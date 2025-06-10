use anyhow::Error;
use std::{fs, path::PathBuf};

use crate::Zettel;

pub fn parse_markdown(zettel: &Zettel, dir: PathBuf) -> Result<(String, String), Error> {
    let path = format!("{}/{}.md", dir.display(), zettel.id);

    let content = fs::read_to_string(path)?;
    let re = String::from("---");

    let splitted_contents = content.splitn(3, &re);

    let contents: Vec<_> = splitted_contents
        .into_iter()
        .map(|content| content.to_string())
        .collect();

    let markdown_content = (contents[1].clone(), contents[2].clone());
    Ok(markdown_content)
}
