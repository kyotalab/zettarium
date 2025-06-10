use anyhow::{Context, Result};
use diesel::SqliteConnection;
use std::{
    io::{Write, stdin, stdout},
    path::PathBuf,
};

use crate::{
    Body, FrontMatter, Markdown, archive_zettel, create_zettel, dedup_and_warn,
    ensure_zettel_exists, get_tag_by_zettel_id, list_zettels, parse_markdown,
    presenter::view_markdown_with_style, print_zettels_as_table, remove_zettel, update_zettel,
    write_to_markdown,
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

    // Display
    print_zettels_as_table(conn, &zettels)?;
    Ok(())
}

pub fn zettel_edit_handler(
    conn: &mut SqliteConnection,
    id: &str,
    title: &str,
    type_: &str,
    tags: &Option<Vec<String>>,
) -> Result<()> {
    // 既存タグの取得
    let mut all_tags = get_tag_by_zettel_id(conn, id)?
        .into_iter()
        .map(|t| t.tag_name)
        .collect::<Vec<_>>();

    // 引数で新規タグが指定されている場合
    if let Some(tags) = tags {
        let new_tags: Vec<String> = tags.iter().map(String::from).collect();
        let new_cleaned = dedup_and_warn(new_tags);

        // 統合 & 重複除去（大文字小文字の区別をなくす）
        let mut tag_set = std::collections::HashSet::new();
        all_tags.retain(|t| tag_set.insert(t.to_lowercase()));
        for tag in new_cleaned {
            if tag_set.insert(tag.to_lowercase()) {
                all_tags.push(tag);
            }
        }
    }

    // 更新
    let updated_zettel = update_zettel(conn, id, title, type_, &all_tags)?;

    // FrontMatterにマージ後のタグを設定
    let front_matter = FrontMatter {
        zettel: updated_zettel.clone(),
        tags: all_tags,
    };

    // MarkdownのBodyをファイルから読み込む
    // noteディレクトリのパスを取得 & ファイルパスを生成
    let dir: PathBuf = ".".into();

    // ファイルパスを指定して、ファイルOpen
    let contents = parse_markdown(&updated_zettel, dir.clone())?;
    let body = contents
        .1
        .trim_start_matches('\n')
        .trim_start_matches("\r\n")
        .to_string();

    // Markdown構造体にマッピング
    let markdown = Markdown {
        front_matter,
        body: Body(body),
    };

    // Markdownファイルの更新
    write_to_markdown(&markdown, dir.clone())?;

    Ok(())
}

pub fn zettel_archive_handler(conn: &mut SqliteConnection, id: &str) -> Result<()> {
    let archived_zettel = archive_zettel(conn, &id)?;

    println!("Archived note: {:?}", archived_zettel.id);
    Ok(())
}

pub fn zettel_remove_handler(conn: &mut SqliteConnection, id: &str, force: bool) -> Result<()> {
    let exist_zettel =
        ensure_zettel_exists(conn, id).with_context(|| format!("Note not found: {}", id))?;

    if !force {
        print!(
            "Are you sure you want to delete note {} ({})? [y/N]: ",
            id, exist_zettel.title
        );
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Cancelled.");
            return Ok(());
        }
    }

    let deleted = remove_zettel(conn, id)?;

    if deleted == 0 {
        println!("Warning:  No note was deleted.");
    } else {
        println!("Removed: Note {} has been removed.", id);
    }

    Ok(())
}

pub fn zettel_view_handler(conn: &mut SqliteConnection, id: &str) -> Result<()> {
    // ファイルの存在確認
    let zettel = ensure_zettel_exists(conn, id)?;
    // noteディレクトリのパスを取得 & ファイルパスを生成
    let dir: PathBuf = ".".into();

    // Display
    view_markdown_with_style(&zettel, &dir)?;

    Ok(())
}
