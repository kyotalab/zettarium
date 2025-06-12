use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};
use which::which;

use crate::AppConfig;

pub fn ensure_fzf_installed() -> Result<()> {
    if which("fzf").is_err() {
        anyhow::bail!("fzf not found. Please install it (e.g. `brew install fzf`).");
    }
    Ok(())
}

pub fn run_fzf(zettel_lines: &[String], config: &AppConfig) -> Result<Option<String>> {
    // fzfプロセス開始
    let mut child = Command::new("fzf")
        .arg("--preview")
        .arg(format!(
            "bat --style=plain --color=always {}/$(echo {{}} | cut -d ' ' -f1).md",
            config.paths.zettel_dir
        ))
        .arg("--preview-window=right:70%")
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
