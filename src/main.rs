use clap::Parser;
use razz::cli::Cli;

fn main() {
    let cli = Cli::parse();
    cli.run();
}
