use std::fs;
use owo_colors::OwoColorize;
use similar::{ChangeTag, TextDiff};

use razz_compiler::{ast::{Program, Spanned}, common::{Position, Span}, compiler::{compiler::{Compiler, CompilerOutput, CompilerStage}, error::CompilerError}, parser::error::ParserError};

pub fn load_fixture(path: &str) -> (String, String) {
    let input = fs::read_to_string(format!("{}/input.rz", path))
        .unwrap();
    let expected = std::fs::read_to_string(format!("{}/expected.txt", path))
        .unwrap_or_default()
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n");
    (input, expected)
}

pub fn run_lexer(input: &str) -> String {
    let compiler = Compiler::new(CompilerStage::Lexer);
    let output = compiler.compiles(input);
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

pub fn run_parser(input: &str) -> Result<Program, Vec<ParserError>> {
    let compiler = Compiler::new(CompilerStage::Parser);
    match compiler.compiles(input) {
        Ok(CompilerOutput::Parser(p)) => Ok(p),
        Ok(_) => panic!("Compiler flag mismatch"),
        Err(CompilerError::Parser(e)) => Err(e),
        Err(_) => panic!("Lexer error"),
    }
}

pub fn s<T>(node: T) -> Spanned<T> {
    Spanned { node, span: Span { 
        start: Position { line: 0, col: 0 }, 
        end: Position { line: 0, col: 0 } 
    }}
}

pub fn colored_assert(actual: &str, expected: &str) {
    if actual != expected {
        let diff = TextDiff::from_lines(actual, expected);

        println!("{}", "===== DIFF (Actual vs Expected) ====".yellow().bold());

        for change in diff.iter_all_changes() {
            match change.tag() {
                // Deletions from 'actual' (what was there but shouldn't be)
                ChangeTag::Delete => println!("{}{}", "-".red(), change.value().red()),
                // Additions from 'expected' (what should have been there)
                ChangeTag::Insert => println!("{}{}", "+".green(), change.value().green()),
                // Equal parts
                ChangeTag::Equal => println!(" {}", change.value()),
            };
        }
        
        println!("\n");
        panic!("{}", "Assertion failed".red().bold());
    }
}
