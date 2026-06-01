use std::{fs, io};

use clap::Parser;
use razz_compiler::{cli::Cli, compiler::{compiler::{Compiler,CompilerOutput}, error::CompilerError}};
use owo_colors::OwoColorize;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let contents = fs::read_to_string(cli.path)?;
    let compiler = Compiler::new(cli.debug);
    let output = compiler.compiles(&contents);
    let mut is_err = false;
    match output {
        Ok(CompilerOutput::Lexer(tokens)) => {
            for token in tokens {
                println!("{}", token.to_string());
            }
        }
        Err(CompilerError::Lexer(err_toks)) => {
            is_err = true;
            for e in err_toks {
                eprintln!("{}: {:?} Start Line: {} Col: {}, End Line: {} Col: {}", "error".red().bold(),
                e.kind, e.span.start.line, e.span.start.col, e.span.end.line, e.span.end.col)
            }
        }

        Ok(CompilerOutput::Parser(_prog)) => {
        }

        Err(CompilerError::Parser(err)) => {
            is_err = true;
            for e in err {
                eprintln!("{}: {:?}", "error".red().bold(), e);
            }
        }

        Ok(CompilerOutput::SSAIR(ir_prog)) => print!("{}", ir_prog),

        // Add more when the compiler grows 
        _ => {}
    }

    if is_err {
        println!("{}: {}", "error".red().bold(), "could not compiled due to previous errors");
    } else {
        println!("{}", "Finished".green().bold());
    }

    Ok(())
}
