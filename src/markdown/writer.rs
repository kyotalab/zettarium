use crate::model::Markdown;
use crate::{AppConfig, ensure_zettel_exists};
use anyhow::Result;
use diesel::SqliteConnection;
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

pub fn edit_with_editor(conn: &mut SqliteConnection, id: &str, config: &AppConfig) -> Result<()> {
    let zettel = ensure_zettel_exists(conn, id)?;
    let path = PathBuf::from(format!("{}/{}.md", &config.paths.zettel_dir, zettel.id));

    let editor = &config.editor;
    let status = std::process::Command::new(editor).arg(&path).status()?;

    if !status.success() {
        println!("Edit was canceled.");
        return Ok(());
    }

    // File update completed (you can sync front matter & DB here if needed)
    println!("Edit completed: {}", path.display());
    Ok(())
}
