use std::fs;
use owo_colors::OwoColorize;
use similar::{ChangeTag, TextDiff};

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

pub fn colored_assert(actual: &str, expected: &str) {
    if actual != expected {
        let diff = TextDiff::from_lines(actual, expected);

        println!("{}", "===== DIFF (Actual vs Expected) ====".yellow().bold());

        for change in diff.iter_all_changes() {
            match change.tag() {
                // Deletions from 'actual' (what was there but shouldn't be)
                ChangeTag::Delete => print!("{}{}", "-".red(), change.value().red()),
                // Additions from 'expected' (what should have been there)
                ChangeTag::Insert => print!("{}{}", "+".green(), change.value().green()),
                // Equal parts
                ChangeTag::Equal => print!(" {}", change.value()),
            };
        }
        
        println!("\n");
        panic!("{}", "Assertion failed".red().bold());
    }
}
