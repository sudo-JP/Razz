//! SSA IR lowering tests
//! Compare against the canonical SSA display format.

use razz_compiler::compiler::{
    compiler::{Compiler, CompilerOutput, CompilerStage},
    error::CompilerError,
};
use razz_compiler::ir::basic_block::BasicBlock;
use razz_compiler::ir::ssa::{SSAInstruction, SSATerminator};

#[cfg(test)]
mod common;
use common::{colored_assert_debug, load_fixture};

type SSABlock = BasicBlock<SSAInstruction, SSATerminator>;

fn run_ir(input: &str) -> Vec<SSABlock> {
    let compiler = Compiler::new(CompilerStage::IR);
    match compiler.compiles(input) {
        Ok(CompilerOutput::IR(blocks)) => blocks,
        Ok(_) => panic!("Compiler flag mismatch"),
        Err(CompilerError::Lexer(errors)) => panic!("Unexpected lexer error: {:?}", errors),
        Err(CompilerError::Parser(errors)) => panic!("Unexpected parser error: {:?}", errors),
        Err(CompilerError::SemanticAnalysis(errors)) => {
            panic!("Unexpected semantic error: {:?}", errors)
        }
        Err(_) => panic!("Unexpected compiler stage error"),
    }
}

fn format_ir(blocks: &[SSABlock]) -> String {
    blocks
        .iter()
        .map(|block| block.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn simple_binop_lowering() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/simple_binop");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn if_else_phi_lowering() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/if_else_phi");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn while_loop_phi_lowering() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/while_loop_phi");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn call_and_unary_lowering() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/call_and_unary");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn for_loop_phi_lowering() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/for_loop_phi");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert_debug(&actual, &expected);
}

#[test]
fn field_compound_assign_lowering() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/field_compound_assign");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert_debug(&actual, &expected);
}

macro_rules! ssa_fixture_test {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            let (input, expected) = load_fixture($path);
            let blocks = run_ir(&input);
            let actual = format_ir(&blocks);
            colored_assert_debug(&actual, &expected);
        }
    };
}

ssa_fixture_test!(advanced_exprs_lowering, "tests/fixtures/ssa_ir/advanced_exprs");
ssa_fixture_test!(
    advanced_http_scene_lowering,
    "tests/fixtures/ssa_ir/advanced_http_scene"
);

ssa_fixture_test!(
    assign_type_annotation_lowering,
    "tests/fixtures/ssa_ir/assign_type_annotation"
);
ssa_fixture_test!(else_if_chain_lowering, "tests/fixtures/ssa_ir/else_if_chain");
ssa_fixture_test!(nested_field_assign_lowering, "tests/fixtures/ssa_ir/nested_field_assign");
ssa_fixture_test!(if_without_else_lowering, "tests/fixtures/ssa_ir/if_without_else");
ssa_fixture_test!(for_no_condition_lowering, "tests/fixtures/ssa_ir/for_no_condition");
ssa_fixture_test!(for_no_decl_lowering, "tests/fixtures/ssa_ir/for_no_decl");
ssa_fixture_test!(nested_while_lowering, "tests/fixtures/ssa_ir/nested_while");
ssa_fixture_test!(nested_for_lowering, "tests/fixtures/ssa_ir/nested_for");
ssa_fixture_test!(nested_if_lowering, "tests/fixtures/ssa_ir/nested_if");
