use anyhow::Result;
use clap::Parser;
use zettarium::{Cli, cli, establish_connection, load_config};

fn main() -> Result<()> {
    let config = load_config().unwrap_or_else(|e| {
        eprintln!("Failed to load config: {}", e);
        std::process::exit(1);
    });

    let conn = &mut establish_connection(&config);
    let cli = Cli::parse();
    cli::dispatch(cli, conn, &config)?;

    Ok(())
}
