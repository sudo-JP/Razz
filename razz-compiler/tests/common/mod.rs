use std::fs;
use owo_colors::OwoColorize;

use razz_compiler::{compiler::compiler::{Compiler, CompilerError, CompilerOutput, CompilerStage}};

pub fn load_fixture(path: &str) -> (String, String) {
    let input = fs::read_to_string(format!("{}/input.rz", path))
        .unwrap();
    let expected = std::fs::read_to_string(format!("{}/expected.txt", path))
        .unwrap()
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    (input, expected)
}


pub fn run_lexer(input: &str) -> String {
    let output = Compiler::compiles(input, CompilerStage::Lexer);
    match output {
        Ok(CompilerOutput::Lexer(tokens)) => 
            tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        Err(CompilerError::Lexer(errors)) => 
            errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => unreachable!("{}", "FAILED TO RUN LEXER".red().bold())
    }
}

pub fn colored_assert(actual: &str, expected: &str) {
    if actual != expected {
        // Print both for debugging
        println!("{}\n{}\n", "===== ACTUAL ====".red().bold(), actual);
        println!("{}\n{}\n", "==== EXPECTED ====".blue().bold(),expected);
        // Panic after printing
        panic!("{}", "Assertion failed".red().bold());
    } 
}
