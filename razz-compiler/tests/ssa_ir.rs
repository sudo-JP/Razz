//! SSA IR lowering tests
//! Compare against the canonical SSA display format.

use razz_compiler::compiler::{
    compiler::{Compiler, CompilerOutput, CompilerStage},
    error::CompilerError,
};
use razz_compiler::ir::ssa::ssa::SSAProgram;

#[cfg(test)]
mod common;
use common::{colored_assert, load_fixture};

fn run_ir(input: &str) -> SSAProgram {
    let compiler = Compiler::new(CompilerStage::SSAIR);
    match compiler.compiles(input) {
        Ok(CompilerOutput::SSAIR(blocks)) => blocks,
        Ok(_) => panic!("Compiler flag mismatch"),
        Err(CompilerError::Lexer(errors)) => panic!("Unexpected lexer error: {:?}", errors),
        Err(CompilerError::Parser(errors)) => panic!("Unexpected parser error: {:?}", errors),
        Err(CompilerError::SemanticAnalysis(errors)) => {
            panic!("Unexpected semantic error: {:?}", errors)
        }
        Err(_) => panic!("Unexpected compiler stage error"),
    }
}

fn format_ir(program: &SSAProgram) -> String {
    program.to_string().trim_end().to_string()
}

#[test]
fn simple_binop() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/simple_binop");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert(&actual, &expected);
}

#[test]
fn if_else_phi() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/if_else_phi");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert(&actual, &expected);
}

#[test]
fn while_loop_phi() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/while_loop_phi");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert(&actual, &expected);
}

#[test]
fn call_and_unary() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/call_and_unary");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert(&actual, &expected);
}

#[test]
fn for_loop_phi() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/for_loop_phi");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert(&actual, &expected);
}

#[test]
fn field_compound_assign() {
    let (input, expected) = load_fixture("tests/fixtures/ssa_ir/field_compound_assign");
    let blocks = run_ir(&input);
    let actual = format_ir(&blocks);
    colored_assert(&actual, &expected);
}

macro_rules! ssa_fixture_test {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            let (input, expected) = load_fixture($path);
            let blocks = run_ir(&input);
            let actual = format_ir(&blocks);
            colored_assert(&actual, &expected);
        }
    };
}

ssa_fixture_test!(advanced_exprs, "tests/fixtures/ssa_ir/advanced_exprs");
ssa_fixture_test!(
    advanced_http_scene,
    "tests/fixtures/ssa_ir/advanced_http_scene"
);

ssa_fixture_test!(
    assign_type_annotation,
    "tests/fixtures/ssa_ir/assign_type_annotation"
);
ssa_fixture_test!(else_if_chain, "tests/fixtures/ssa_ir/else_if_chain");
ssa_fixture_test!(nested_field_assign, "tests/fixtures/ssa_ir/nested_field_assign");
ssa_fixture_test!(if_without_else, "tests/fixtures/ssa_ir/if_without_else");
ssa_fixture_test!(for_no_condition, "tests/fixtures/ssa_ir/for_no_condition");
ssa_fixture_test!(for_no_decl, "tests/fixtures/ssa_ir/for_no_decl");
ssa_fixture_test!(nested_while, "tests/fixtures/ssa_ir/nested_while");
ssa_fixture_test!(nested_for, "tests/fixtures/ssa_ir/nested_for");
ssa_fixture_test!(nested_if, "tests/fixtures/ssa_ir/nested_if");
ssa_fixture_test!(multiple_phis_merging_path, "tests/fixtures/ssa_ir/multiple_phis_merging_path");

// KNOWN BUG in `ssa_lowerer.rs`: named-argument call sites are lowered in the
// ORDER THEY'RE WRITTEN at the call site, not reordered to match the callee's
// declared parameter order. Semantic analysis (`analyzer.rs`) validates arg
// names/types/duplicates via a name->type map and explicitly allows any
// argument order (that's the entire point of Swift-style named args), but
// that name info is discarded before SSA lowering, which just does
// `args.iter().map(|arg| self.lower_expr(&arg.expr))` positionally.
//
// Fixture: `fn sub(a: int, b: int) int { return a - b; }` called as
// `sub(b: 1, a: 10)` (a=10, b=1 by name, so the correct result is 10-1=9).
// It currently lowers to `t3 = sub(1, 10)` -- silently swapping the values
// bound to `a` and `b` -- instead of the correct `t3 = sub(10, 1)`.
// This test asserts the correct/reordered lowering and will FAIL until the
// lowerer resolves each arg's target parameter index by name before emitting
// the `Call` instruction's `args` list.
ssa_fixture_test!(named_arg_reorder, "tests/fixtures/ssa_ir/named_arg_reorder");
