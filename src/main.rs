use clap::Parser;
use zettarium::{Cli, cli};

fn main() {
    let cli = Cli::parse();
    cli::dispatch(cli);
}
