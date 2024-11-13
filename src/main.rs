use clap::Parser;
use olymp::*;

fn main() {
    let cli = Cli::parse();

    cli.solve();
}
