use std::{fs, io};

use clap::Parser;
use razz_compiler::{cli::Cli, compiler::compiler::{Compiler, CompilerError, CompilerOutput}};
use owo_colors::OwoColorize;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let contents = fs::read_to_string(cli.path)?;
    let output = Compiler::compiles(&contents, cli.compile);
    match output {
        Ok(CompilerOutput::Lexer(tokens)) => {
            for token in tokens {
                println!("{}", token.to_string());
            }
        }
        Err(CompilerError::Lexer(err_toks)) => {
            for token in err_toks {
                println!("{}", token.to_string());
            }
        }

        // Add more when the compiler grows 
        _ => {}
    }

    println!("{}", "Finished".green().bold());

    Ok(())
}
