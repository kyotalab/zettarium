use anyhow::{Context, Result};
use diesel::SqliteConnection;
use std::io::{Write, stdin, stdout};

use crate::{
    AppConfig, Body, FrontMatter, Markdown, archive_zettel, create_zettel, dedup_and_warn,
    edit_with_editor, ensure_zettel_exists, get_tag_by_zettel_id, list_zettels,
    presenter::view_markdown_with_style, print_zettels_as_table, remove_zettel,
    update_markdown_file, update_zettel, write_to_markdown,
};

pub fn zettel_new_handler(
    conn: &mut SqliteConnection,
    title: &str,
    type_: &str,
    tags: &Option<Vec<String>>,
    config: &AppConfig,
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
        zettel: zettel.clone(),
        tags: cleaned_tags.clone(),
    };

    // MarkdownのBody生成
    let body = Body(format!("## {}", front_matter.get_zettel().title));

    // Markdown構造体にマッピング
    let markdown = Markdown { front_matter, body };

    // Markdownファイルの生成
    let dir = &config.paths.zettel_dir;
    write_to_markdown(&markdown, dir.into())?;

    // エディタを開いて編集
    let edited_zettel = edit_with_editor(conn, &zettel.id, config)?;

    // 編集後のタグは変更なしなので再利用
    update_markdown_file(&edited_zettel, &cleaned_tags, dir)?;

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
    config: &AppConfig,
) -> Result<()> {
    // --------------------------------------
    // モード1: エディタを開いて編集する
    // --------------------------------------
    if title.is_none() && type_.is_none() && tags.is_none() {
        let updated = edit_with_editor(conn, id, config)?;
        let tags = get_tag_by_zettel_id(conn, id)?
            .into_iter()
            .map(|t| t.tag_name)
            .collect::<Vec<_>>();

        update_markdown_file(&updated, &tags, &config.paths.zettel_dir)?;
        return Ok(());
    }

    // --------------------------------------
    // モード2: CLIオプションで部分更新
    // --------------------------------------
    let existing_zettel = ensure_zettel_exists(conn, id)?;

    let merged_tags = merge_tags(conn, id, tags.clone())?;

    let final_title = title.unwrap_or(&existing_zettel.title);
    let final_type = type_
        .map(|t| t.to_string())
        .unwrap_or_else(|| format!("{:?}", existing_zettel.type_));

    let updated_zettel = update_zettel(conn, id, final_title, &final_type, &merged_tags)?;

    // Markdown更新処理
    update_markdown_file(&updated_zettel, &merged_tags, &config.paths.zettel_dir)?;

    Ok(())
}

fn merge_tags(
    conn: &mut SqliteConnection,
    zettel_id: &str,
    new_tags: Option<Vec<String>>,
) -> Result<Vec<String>> {
    let mut all_tags = get_tag_by_zettel_id(conn, zettel_id)?
        .into_iter()
        .map(|t| t.tag_name)
        .collect::<Vec<_>>();

    if let Some(input_tags) = new_tags {
        let cleaned = dedup_and_warn(input_tags);
        let mut seen = std::collections::HashSet::new();
        all_tags.retain(|t| seen.insert(t.to_lowercase()));
        for tag in cleaned {
            if seen.insert(tag.to_lowercase()) {
                all_tags.push(tag);
            }
        }
    }

    Ok(all_tags)
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

pub fn zettel_view_handler(
    conn: &mut SqliteConnection,
    id: &str,
    config: &AppConfig,
) -> Result<()> {
    // ファイルの存在確認
    let zettel = ensure_zettel_exists(conn, id)?;
    // noteディレクトリのパスを取得 & ファイルパスを生成
    let dir = &config.paths.zettel_dir;

    // Display
    view_markdown_with_style(&zettel, dir.into())?;

    Ok(())
}
