use crate::model::Markdown;
use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{fs, path::PathBuf};

pub fn write_to_markdown(markdown: &Markdown, dir: PathBuf) -> Result<()> {
    fs::create_dir_all(&dir)?;

    let filename = format!("{}.md", markdown.get_front_matter().get_zettel().id);
    let path = dir.join(filename);

    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "{}", markdown)?;

    println!("Markdown saved to {}", path.display());
    Ok(())
}
