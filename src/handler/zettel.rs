use anyhow::Result;
use diesel::SqliteConnection;

use crate::{
    Body, FrontMatter, Markdown, create_zettel, dedup_and_warn, list_zettels, write_to_markdown,
};

pub fn zettel_new_handler(
    conn: &mut SqliteConnection,
    title: &str,
    type_: &str,
    tags: &Option<Vec<String>>,
) -> Result<()> {
    // tag重複確認
    let mut tags_str: Vec<String> = vec![];
    if let Some(tags) = tags {
        tags_str = tags.into_iter().map(String::from).collect();
    }
    let cleaned_tags = dedup_and_warn(tags_str);

    // Zettel構造体にマッピングしてSQLiteに保存
    let zettel = create_zettel(conn, title, type_, &cleaned_tags)?;

    // FrontMatter構造体にマッピング
    let front_matter = FrontMatter {
        zettel,
        tags: cleaned_tags,
    };

    // MarkdownのBody生成
    let body = Body(format!("## {}", front_matter.get_zettel().title));

    // Markdown構造体にマッピング
    let markdown = Markdown { front_matter, body };

    // Markdownファイルの生成
    let dir = ".";
    write_to_markdown(&markdown, dir.into())?;

    Ok(())
}

pub fn zettel_list_handler(
    conn: &mut SqliteConnection,
    id: Option<&str>,
    type_: Option<&str>,
    tags: &Option<Vec<String>>,
    all: bool,
    archived: bool,
) -> Result<()> {
    // tag重複確認
    let mut tags_str: Vec<String> = vec![];
    if let Some(tags) = tags {
        tags_str = tags.into_iter().map(String::from).collect();
    }
    let cleaned_tags = dedup_and_warn(tags_str);

    // Zettel一覧の取得
    let zettels = list_zettels(conn, id, type_, &cleaned_tags, all, archived)?;

    println!("len: {:?}", zettels.len());

    // Display
    for zettel in zettels {
        println!("{}", zettel);
    }
    Ok(())
}
