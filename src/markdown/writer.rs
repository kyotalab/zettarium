use crate::model::Markdown;
use crate::{
    AppConfig, Body, FrontMatter, Zettel, ensure_zettel_exists, parse_markdown,
    update_zettel_timestamp_only,
};
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

pub fn edit_with_editor(
    conn: &mut SqliteConnection,
    id: &str,
    config: &AppConfig,
) -> Result<Zettel> {
    let zettel = ensure_zettel_exists(conn, id)?;
    let path = PathBuf::from(format!("{}/{}.md", &config.paths.zettel_dir, zettel.id));

    let editor = &config.editor.editor;
    let status = std::process::Command::new(editor).arg(&path).status()?;

    if !status.success() {
        println!("Edit was cancelled.");
        anyhow::bail!("Editor exited with non-zero status");
    }

    // `updated_at` だけ更新する
    let zettel = update_zettel_timestamp_only(conn, id)?; // ← 新たに関数を用意

    Ok(zettel)
}

pub fn update_markdown_file(zettel: &Zettel, tags: &[String], dir: &str) -> Result<()> {
    let (_, body_raw) = parse_markdown(zettel, dir.into())?;
    let cleaned_body = body_raw
        .trim_start_matches('\n')
        .trim_start_matches("\r\n")
        .to_string();

    let markdown = Markdown {
        front_matter: FrontMatter {
            zettel: zettel.clone(),
            tags: tags.to_vec(),
        },
        body: Body(cleaned_body),
    };

    write_to_markdown(&markdown, dir.into())
}
