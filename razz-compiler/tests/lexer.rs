mod common;

#[test]
fn delimiters() {
    let (input, expected) = common::load_fixture("tests/fixtures/lexer/delimiters");
    let actual = common::run_lexer(&input);
    common::colored_assert(&actual, &expected);
}

#[test]
fn operators_with_no_div() {
    let (input, expected) = common::load_fixture("tests/fixtures/lexer/operators_with_no_div");
    let actual = common::run_lexer(&input);
    common::colored_assert(&actual, &expected);
}

#[test]
fn literals_basic() {
    let (input, expected) = common::load_fixture("tests/fixtures/lexer/literals_basic");
    let actual = common::run_lexer(&input);
    common::colored_assert(&actual, &expected);
}

#[test]
fn literals_basic_invalid() {
    let (input, expected) = common::load_fixture("tests/fixtures/lexer/literals_basic_invalid");
    let actual = common::run_lexer(&input);
    common::colored_assert(&actual, &expected);
}

#[test]
fn slash_handling() {
    let (input, expected) = common::load_fixture("tests/fixtures/lexer/slash_handling");
    let actual = common::run_lexer(&input);
    common::colored_assert(&actual, &expected);
}

#[test]
fn slash_invalid() {
    let (input, expected) = common::load_fixture("tests/fixtures/lexer/slash_invalid");
    let actual = common::run_lexer(&input);
    common::colored_assert(&actual, &expected);
}

#[test]
fn verbose() {
    let (input, expected) = common::load_fixture("tests/fixtures/lexer/verbose");
    let actual = common::run_lexer(&input);
    common::colored_assert(&actual, &expected);
}

#[test]
fn endpoints_valid() {
    let (input, expected) = common::load_fixture("tests/fixtures/lexer/endpoints_valid");
    let actual = common::run_lexer(&input);
    common::colored_assert(&actual, &expected);
}
