use anyhow::Result;
use clap::Parser;
use zettarium::{Cli, cli};

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli::dispatch(cli)?;

    Ok(())
}
