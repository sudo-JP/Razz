#[cfg(test)]
mod common;

use razz_compiler::compiler::{
    compiler::{Compiler, CompilerOutput, CompilerStage},
    error::CompilerError,
};
use common::{colored_assert_debug, load_fixture};

fn run_lexer(input: &str) -> String {
    let compiler = Compiler::new(CompilerStage::Lexer);
    match compiler.compiles(input) {
        Ok(CompilerOutput::Lexer(tokens)) => tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        Err(CompilerError::Lexer(errors)) => errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => panic!("FAILED TO RUN LEXER"),
    }
}

#[test]
fn delimiters() {
    let (input, expected) = load_fixture("tests/fixtures/lexer/delimiters");
    let actual = run_lexer(&input);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn operators_with_no_div() {
    let (input, expected) = load_fixture("tests/fixtures/lexer/operators_with_no_div");
    let actual = run_lexer(&input);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn literals_basic() {
    let (input, expected) = load_fixture("tests/fixtures/lexer/literals_basic");
    let actual = run_lexer(&input);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn literals_basic_invalid() {
    let (input, expected) = load_fixture("tests/fixtures/lexer/literals_basic_invalid");
    let actual = run_lexer(&input);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn slash_handling() {
    let (input, expected) = load_fixture("tests/fixtures/lexer/slash_handling");
    let actual = run_lexer(&input);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn slash_invalid() {
    let (input, expected) = load_fixture("tests/fixtures/lexer/slash_invalid");
    let actual = run_lexer(&input);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn verbose() {
    let (input, expected) = load_fixture("tests/fixtures/lexer/verbose");
    let actual = run_lexer(&input);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn endpoints_valid() {
    let (input, expected) = load_fixture("tests/fixtures/lexer/endpoints_valid");
    let actual = run_lexer(&input);
    colored_assert_debug(&actual, &expected);
}
