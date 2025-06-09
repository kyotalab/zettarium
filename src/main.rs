use anyhow::Result;
use clap::Parser;
use zettarium::{Cli, cli, establish_connection};

fn main() -> Result<()> {
    let conn = &mut establish_connection();
    let cli = Cli::parse();
    cli::dispatch(cli, conn)?;

    Ok(())
}
