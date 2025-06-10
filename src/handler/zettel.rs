use anyhow::{Context, Result};
use diesel::SqliteConnection;
use std::{
    io::{Write, stdin, stdout},
    path::PathBuf,
};

use crate::{
    Body, FrontMatter, Markdown, archive_zettel, create_zettel, dedup_and_warn, edit_with_editor,
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
    title: Option<&str>,
    type_: Option<&str>,
    tags: &Option<Vec<String>>,
) -> Result<()> {
    // --------------------------------------
    // モード1: エディタを開いて編集する
    // --------------------------------------
    if title.is_none() && type_.is_none() && tags.is_none() {
        return edit_with_editor(conn, id);
    }

    // --------------------------------------
    // モード2: CLIオプションで部分更新
    // --------------------------------------

    // 既存Zettel取得
    let existing_zettel = ensure_zettel_exists(conn, id)?;

    // タグの統合（既存タグ + CLIタグ）
    let mut all_tags = get_tag_by_zettel_id(conn, id)?
        .into_iter()
        .map(|t| t.tag_name)
        .collect::<Vec<_>>();

    if let Some(tags) = tags {
        let new_cleaned = dedup_and_warn(tags.clone());

        let mut tag_set = std::collections::HashSet::new();
        all_tags.retain(|t| tag_set.insert(t.to_lowercase()));
        for tag in new_cleaned {
            if tag_set.insert(tag.to_lowercase()) {
                all_tags.push(tag);
            }
        }
    }

    // title/type_ の値は存在しない場合、既存値を使う
    let new_title = title.unwrap_or(&existing_zettel.title);
    let new_type = type_
        .map(|t| t.to_string())
        .unwrap_or_else(|| format!("{:?}", existing_zettel.type_));

    // DB更新
    let updated_zettel = update_zettel(conn, id, new_title, &new_type, &all_tags)?;

    // MarkdownのBodyをファイルから取得
    let dir: PathBuf = ".".into();
    let (_, body_raw) = parse_markdown(&updated_zettel, dir.clone())?;
    let cleaned_body = body_raw
        .trim_start_matches('\n')
        .trim_start_matches("\r\n")
        .to_string();

    // Markdown再生成
    let markdown = Markdown {
        front_matter: FrontMatter {
            zettel: updated_zettel,
            tags: all_tags,
        },
        body: Body(cleaned_body),
    };

    write_to_markdown(&markdown, dir)?;

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
