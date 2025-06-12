use anyhow::Result;
use clap::Parser;
use zettarium::{Cli, Commands, cli, establish_connection, load_config};

fn main() -> Result<()> {
    let cli = Cli::parse();
    if matches!(cli.command, Commands::Init) {
        // configが最低限必要ならここで読み込む（なければデフォルトでもOK）
        let config = load_config().unwrap_or_else(|e| {
            eprintln!("Failed to load config: {}", e);
            std::process::exit(1);
        });

        cli::dispatch(cli, None, &config)?; // DB接続なしで渡す
        return Ok(());
    }

    let config = load_config().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}", e);
        std::process::exit(1);
    });

    let conn = &mut establish_connection(&config);
    cli::dispatch(cli, Some(conn), &config)?;

    Ok(())
}
