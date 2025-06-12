use anyhow::{Context, Result};
use diesel::SqliteConnection;
use std::io::Write;
use std::process::{Command, Stdio};
use which::which;

use crate::{AppConfig, Zettel, list_zettels, parse_markdown};

pub fn ensure_fzf_installed() -> Result<()> {
    if which("fzf").is_err() {
        anyhow::bail!("fzf not found. Please install it (e.g. `brew install fzf`).");
    }
    Ok(())
}

pub fn run_fzf(zettel_lines: &[String], config: &AppConfig) -> Result<Option<String>> {
    // fzfプロセス開始
    let mut child = Command::new("fzf")
        .args(&[
            "--ansi",
            "--with-nth=3..", // 3列目以降（タイトルなど）を対象に検索
            "--delimiter=|",
            "--preview",
            &format!(
                "bat --style=plain --color=always {}/$(echo {{}} | cut -d ' ' -f1).md",
                config.paths.zettel_dir
            ),
            "--preview-window=right:70%",
            "--prompt",
            "🔍 Search: ",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to spawn fzf")?;

    // fzfの標準入力に候補を書き込む
    {
        let stdin = child.stdin.as_mut().context("Failed to open fzf stdin")?;
        for line in zettel_lines {
            writeln!(stdin, "{}", line)?;
        }
    }

    // fzfの出力（選択結果）を受け取る
    let output = child
        .wait_with_output()
        .context("Failed to read fzf output")?;

    if output.status.success() {
        let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(Some(selected))
    } else {
        Ok(None) // ユーザーがキャンセルしたなど
    }
}

pub fn find_backlinks(
    conn: &mut SqliteConnection,
    target_id: &str,
    config: &AppConfig,
) -> Result<Vec<Zettel>> {
    let all_zettels = list_zettels(conn, None, None, &[], true, false)?;
    let mut backlinks = vec![];

    for z in &all_zettels {
        let (_, body) = parse_markdown(z, config.paths.zettel_dir.clone().into())?;
        if body.contains(&format!("./{}.md", target_id)) {
            backlinks.push(z.clone());
        }
    }

    Ok(backlinks)
}
