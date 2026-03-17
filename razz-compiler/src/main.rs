use std::{fs, io};

use clap::Parser;
use razz_compiler::{cli::Cli, compiler::compiler::Compiler};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let contents = fs::read_to_string(cli.path)?;
    Compiler::compiles(&contents, cli.compile);

    Ok(())
}
