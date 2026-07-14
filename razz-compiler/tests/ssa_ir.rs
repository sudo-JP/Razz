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
ssa_fixture_test!(named_arg_reorder, "tests/fixtures/ssa_ir/named_arg_reorder");

// KNOWN BUG in `ssa_lowerer.rs`'s `lower_if` (around lines 671-775): whenever a
// conditional has no final `else` (a bare `if` with no else, OR an else-if
// chain that doesn't end in `else`), the "false" edge that skips straight to
// the merge block (`exit_id`) is never registered as a CFG predecessor via
// `add_pred`. Every `add_pred(exit_id, ...)` call in that function happens
// AFTER a taken-branch body finishes (the `if`-body, each `else-if`-body, or
// the `else`-body) -- there is no `add_pred(exit_id, header_id)` (or
// `add_pred(exit_id, last_elif_header)`) for the direct "condition was false,
// nothing ran" path. Since SSA's `read_variable_recursive` walks predecessors
// to build phi nodes, this missing edge makes it blind to the "branch not
// taken" value: it either fabricates NO phi at all (silently returning
// whatever the taken branch computed, unconditionally -- see
// `if_no_else_used_after` below) or an INCOMPLETE phi missing one incoming
// value (see `elseif_chain_no_final_else`, which currently crashes further
// downstream in the HIR structurizer as a direct consequence). This is one
// root-cause bug with several distinct symptoms across the 3 tests below --
// fixing `lower_if` to register the missing predecessor edge(s) should
// resolve all three at once.

/// Simplest reproduction: `if n > 3 { dummy = 1; } return dummy;` with a
/// genuinely runtime (non-constant) condition and `dummy` used afterward.
/// Currently compiles to an unconditional `ret 1` (see the SSA dump: no phi
/// is generated at all), silently discarding the initial `dummy = 0` value
/// whenever the condition is false. This is a SILENT CORRECTNESS bug (no
/// panic) -- the most dangerous kind, since it produces wrong output with no
/// indication anything failed.
ssa_fixture_test!(if_no_else_used_after, "tests/fixtures/ssa_ir/if_no_else_used_after");

/// Else-if chain with no final `else`: `if n>10 {result=1} else if n>5
/// {result=2}` then `return result`. The middle else-if header's own
/// direct-false-edge to the merge block is missing the same way, so the
/// generated phi is missing its third incoming value (the "neither branch
/// taken" case, `result` staying `0`). Confirmed via `--debug ssair` that the
/// SSA here already carries a malformed/incomplete `Phi` (only 2 of 3
/// possible incoming values), which then makes the HIR structurizer PANIC
/// with `"phi resolution should never walk into a return"`
/// (`hir_structurizer.rs:603`) trying to resolve it. Root cause is upstream in
/// SSA, same as `if_no_else_used_after`.
ssa_fixture_test!(elseif_chain_no_final_else, "tests/fixtures/ssa_ir/elseif_chain_no_final_else");

/// Same root cause, nested inside a `for` loop: `if i > 2 { total = total +
/// i; }` inside the loop body, with `total` also being the loop-carried
/// accumulator. The inner if's exit block is missing the phi merging "total
/// unchanged" (condition false) vs "total + i" (condition true), so the loop
/// back-edge's phi for `total` incorrectly reads the inner-if-body's temp
/// directly instead of a properly merged value -- meaning the accumulator can
/// silently use a stale/wrong value on iterations where the inner condition
/// is false. The expected SSA below is a best-effort ideal reconstruction
/// (adding the missing phi at the inner if's exit block); exact temp
/// numbering may shift once the underlying `lower_if` fix lands -- update as
/// needed.
ssa_fixture_test!(if_no_else_in_loop, "tests/fixtures/ssa_ir/if_no_else_in_loop");
