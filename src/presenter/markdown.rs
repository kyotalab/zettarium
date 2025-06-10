use crate::Zettel;
use anyhow::{Context, Result};
use std::{fs, path::PathBuf};
use termimad::{CompoundStyle, MadSkin, StyledChar, rgb};

pub fn view_markdown_with_style(zettel: &Zettel, dir: PathBuf) -> Result<()> {
    // ファイルパスを指定して、ファイルOpen
    let filename = format!("{}.md", zettel.id);
    let path = dir.join(filename);

    // Markdownファイルをレンダリングする処理
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read markdown file at {}", path.display()))?;
    let skin = create_custom_skin();
    skin.print_text(&content);

    Ok(())
}

pub fn create_custom_skin() -> MadSkin {
    let mut skin = MadSkin::default();

    // ────────────── ヘッダー設定 ──────────────
    skin.headers[0].set_fg(rgb(255, 215, 0)); // H1 - ゴールド
    skin.headers[1].set_fg(rgb(255, 200, 0)); // H2 - 明るいオレンジ
    skin.headers[2].set_fg(rgb(255, 165, 0)); // H3 - オレンジ
    skin.headers[3].set_fg(rgb(255, 140, 0)); // H4 - ダークオレンジ
    skin.headers[4].set_fg(rgb(255, 120, 0)); // H5 - ブロンズ
    skin.headers[5].set_fg(rgb(255, 100, 0)); // H6 - 深いブロンズ

    // ────────────── 太字・斜体 ──────────────
    skin.bold.set_fg(rgb(255, 255, 0)); // 黄色
    skin.italic.set_fg(rgb(0, 255, 127)); // 明るい緑

    // ────────────── 箇条書き ──────────────
    let bullet_style = CompoundStyle::with_fg(rgb(0, 200, 255)); // シアン
    skin.bullet = StyledChar::new(bullet_style, '•');

    // ────────────── 引用 ──────────────
    let quote_style = CompoundStyle::with_fg(rgb(180, 180, 180));
    skin.quote_mark = StyledChar::new(quote_style, '┃');

    // ────────────── 水平線 ──────────────
    skin.paragraph.set_fg(rgb(100, 100, 100));

    // ────────────── コードブロック ──────────────
    skin.code_block.set_fg(rgb(173, 216, 230)); // ライトブルー
    skin.code_block.set_bg(rgb(30, 30, 30)); // ダークグレー

    // ────────────── インラインコード ──────────────
    skin.inline_code.set_fg(rgb(255, 255, 255));
    skin.inline_code.set_bg(rgb(50, 50, 50));

    // ────────────── 取り消し線 ──────────────
    skin.strikeout.set_fg(rgb(150, 150, 150));

    skin
}
