use std::fs;
use owo_colors::OwoColorize;

use razz_compiler::lexer::{lexer::Lexer, tokens::{Token, TokenKind}};

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

fn format_token(token: &Token) -> String {
    match token.kind {
        TokenKind::Eof => "Eof".to_string(),       
        _ => format!("{:?} Line: {} Col: {}", token.kind, token.line, token.col),
    }
}

pub fn run_lexer(input: &str) -> String {
    match Lexer::new(input).scan_tokens() {
        Ok(tokens) => tokens
            .iter()
            .map(|t| format_token(t))
            .collect::<Vec<_>>()
            .join("\n"),
        Err(errors) => errors
            .iter()
            .map(|e| format!("ERROR: {:?} Line: {} Col: {}", e.kind, e.line, e.col)
                .trim_end()
                .to_string())
            .collect::<Vec<_>>()
            .join("\n"),
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
