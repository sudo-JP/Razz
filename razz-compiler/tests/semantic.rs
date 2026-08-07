//! Semantic analysis tests
//! Keep tests black-box: assert intended language behavior, not implementation details.

use std::collections::{HashMap, HashSet};

use razz_compiler::ast::{NodeId, TypeKind};
use razz_compiler::compiler::{
    compiler::{Compiler, CompilerOutput, CompilerStage},
    error::CompilerError,
};
use razz_compiler::semantic::error::{SemanticError, SemanticErrorKind};

#[cfg(test)]
mod common;
use common::load_fixture;

fn run_semantic(input: &str) -> Result<(HashSet<NodeId>, HashMap<NodeId, TypeKind>), Vec<SemanticError>> {
    let compiler = Compiler::new(CompilerStage::SemanticAnalysis, false);
    match compiler.compiles(input, None) {
        Ok(CompilerOutput::SemanticAnalysis(mutable_set, type_table)) => Ok((mutable_set, type_table)),
        Ok(_) => panic!("Compiler flag mismatch"),
        Err(CompilerError::SemanticAnalysis(errors)) => Err(errors),
        Err(CompilerError::Lexer(errors)) => panic!("Unexpected lexer error: {:?}", errors),
        Err(CompilerError::Parser(errors)) => panic!("Unexpected parser error: {:?}", errors),
        Err(_) => panic!("Unexpected compiler stage error"),
    }
}

fn fixture_input(path: &str) -> String {
    let (input, _) = load_fixture(path);
    input
}

#[test]
fn simple_addition_ok_and_immutable() {
    let input = fixture_input("tests/fixtures/semantic/simple_addition_ok");
    let (mutable_set, _) = run_semantic(&input).expect("Semantic analysis should pass");
    assert!(mutable_set.is_empty(), "Simple addition should not mark mutability");
}

#[test]
fn string_addition_ok_and_immutable() {
    let input = fixture_input("tests/fixtures/semantic/string_addition_ok");
    let (mutable_set, _) = run_semantic(&input).expect("Semantic analysis should pass");
    assert!(
        mutable_set.is_empty(),
        "String addition without reassignment should stay immutable",
    );
}

#[test]
fn reassignment_marks_mutable() {
    let input = fixture_input("tests/fixtures/semantic/reassignment_mutability_ok");
    let (mutable_set, _) = run_semantic(&input).expect("Semantic analysis should pass");
    assert!(
        !mutable_set.is_empty(),
        "Reassignment should mark at least one mutable variable",
    );
}

#[test]
fn compound_assign_marks_mutable() {
    let input = fixture_input("tests/fixtures/semantic/compound_assign_mutability_ok");
    let (mutable_set, _) = run_semantic(&input).expect("Semantic analysis should pass");
    assert!(
        !mutable_set.is_empty(),
        "Compound assignment should mark at least one mutable variable",
    );
}

#[test]
fn mixed_reassignment_and_compound_assign_count_mutables() {
    let input = fixture_input("tests/fixtures/semantic/mixed_mutability_count_ok");
    let (mutable_set, _) = run_semantic(&input).expect("Semantic analysis should pass");
    assert_eq!(
        mutable_set.len(),
        2,
        "Expected exactly two mutable variables from reassignment/compound assign",
    );
}

#[test]
fn wrong_return_type_errors() {
    let input = fixture_input("tests/fixtures/semantic/wrong_return_type_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::TypeMismatch { .. })),
        "Expected at least one type mismatch error",
    );
}

#[test]
fn missing_return_in_non_null_function_errors() {
    let input = fixture_input("tests/fixtures/semantic/missing_return_non_null_err");
    let result = run_semantic(&input);
    assert!(
        result.is_err(),
        "Non-null functions should fail semantic analysis if no return statement is present",
    );
}

#[test]
fn return_single_statement_ok() {
    let input = fixture_input("tests/fixtures/semantic/return_single_statement_ok");
    run_semantic(&input).expect("Single return should satisfy non-null function return requirement");
}

#[test]
fn return_if_else_all_paths_ok() {
    let input = fixture_input("tests/fixtures/semantic/return_if_else_all_paths_ok");
    run_semantic(&input).expect("if/else with return in both branches should satisfy return requirement");
}

#[test]
fn return_else_if_chain_all_paths_ok() {
    let input = fixture_input("tests/fixtures/semantic/return_else_if_chain_all_paths_ok");
    run_semantic(&input).expect("if/else if/else with returns in all branches should satisfy return requirement");
}

#[test]
fn return_nested_if_all_paths_ok() {
    let input = fixture_input("tests/fixtures/semantic/return_nested_if_all_paths_ok");
    run_semantic(&input).expect("Nested branching with full returns should satisfy return requirement");
}

#[test]
fn missing_return_if_without_else_errors() {
    let input = fixture_input("tests/fixtures/semantic/missing_return_if_without_else_err");
    let result = run_semantic(&input);
    assert!(
        result.is_err(),
        "if without else does not guarantee return for non-null functions",
    );
}

#[test]
fn missing_return_else_if_without_final_else_errors() {
    let input = fixture_input("tests/fixtures/semantic/missing_return_else_if_without_final_else_err");
    let result = run_semantic(&input);
    assert!(
        result.is_err(),
        "if/else if chain without final else does not guarantee return",
    );
}

#[test]
fn missing_return_if_else_missing_branch_errors() {
    let input = fixture_input("tests/fixtures/semantic/missing_return_if_else_missing_branch_err");
    let result = run_semantic(&input);
    assert!(
        result.is_err(),
        "if/else where one branch lacks return should fail non-null return requirement",
    );
}

#[test]
fn while_loop_return_not_guaranteed_errors() {
    let input = fixture_input("tests/fixtures/semantic/while_loop_return_not_guaranteed_err");
    let result = run_semantic(&input);
    assert!(
        result.is_err(),
        "return inside while loop alone does not guarantee function return",
    );
}

#[test]
fn for_loop_return_not_guaranteed_errors() {
    let input = fixture_input("tests/fixtures/semantic/for_loop_return_not_guaranteed_err");
    let result = run_semantic(&input);
    assert!(
        result.is_err(),
        "return inside for loop alone does not guarantee function return",
    );
}

#[test]
fn if_condition_must_be_bool() {
    let input = fixture_input("tests/fixtures/semantic/if_condition_must_be_bool_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidConditionType(_))),
        "Expected invalid condition type error",
    );
}

#[test]
fn while_condition_must_be_bool() {
    let input = fixture_input("tests/fixtures/semantic/while_condition_must_be_bool_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidConditionType(_))),
        "Expected invalid while condition type error",
    );
}

#[test]
fn for_condition_must_be_bool() {
    let input = fixture_input("tests/fixtures/semantic/for_condition_must_be_bool_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidConditionType(_))),
        "Expected invalid for condition type error",
    );
}

#[test]
fn control_flow_bool_conditions_ok() {
    let input = fixture_input("tests/fixtures/semantic/control_flow_bool_conditions_ok");
    run_semantic(&input).expect("Boolean conditions in if/while/for should pass");
}

#[test]
fn unary_ops_ok() {
    let input = fixture_input("tests/fixtures/semantic/unary_ops_ok");
    run_semantic(&input).expect("Valid unary operations should pass");
}

#[test]
fn unary_ops_invalid_error() {
    let input = fixture_input("tests/fixtures/semantic/unary_ops_invalid_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidUnOp { .. })),
        "Expected invalid unary operation error",
    );
}

#[test]
fn bin_ops_valid_coverage_ok() {
    let input = fixture_input("tests/fixtures/semantic/bin_ops_valid_coverage_ok");
    run_semantic(&input).expect("Valid binary operations should pass");
}

#[test]
fn bin_ops_invalid_error() {
    let input = fixture_input("tests/fixtures/semantic/bin_ops_invalid_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors.iter().any(|e| {
            matches!(
                e.kind,
                SemanticErrorKind::InvalidBinOp { .. } | SemanticErrorKind::TypeMismatch { .. }
            )
        }),
        "Expected invalid binary op or type mismatch error",
    );
}

#[test]
fn http_get_endpoint_types_ok() {
    let input = fixture_input("tests/fixtures/semantic/http_get_endpoint_types_ok");
    run_semantic(&input).expect("GET should type-check to endpoint-specific object types");
}

#[test]
fn http_get_invalid_endpoint_error() {
    let input = fixture_input("tests/fixtures/semantic/http_get_invalid_endpoint_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidGetRequest(_))),
        "Expected invalid GET endpoint error",
    );
}

#[test]
fn http_post_hittable_requires_all_fields_ok() {
    let input = fixture_input("tests/fixtures/semantic/http_post_hittable_requires_all_fields_ok");
    run_semantic(&input).expect("POST /hittable should accept complete Sphere body");
}

#[test]
fn http_post_hittable_missing_field_error() {
    let input = fixture_input("tests/fixtures/semantic/http_post_hittable_missing_field_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::MissingField(_))),
        "Expected missing field error for incomplete POST body",
    );
}

#[test]
fn http_put_image_requires_all_fields_ok() {
    let input = fixture_input("tests/fixtures/semantic/http_put_image_requires_all_fields_ok");
    run_semantic(&input).expect("PUT should require and accept full object body");
}

#[test]
fn http_put_image_missing_field_error() {
    let input = fixture_input("tests/fixtures/semantic/http_put_image_missing_field_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::MissingField(_))),
        "Expected missing field error for incomplete PUT body",
    );
}

#[test]
fn http_patch_image_partial_ok() {
    let input = fixture_input("tests/fixtures/semantic/http_patch_image_partial_ok");
    run_semantic(&input).expect("PATCH should allow partial object updates");
}

#[test]
fn field_access_valid_ok() {
    let input = fixture_input("tests/fixtures/semantic/field_access_valid_ok");
    run_semantic(&input).expect("Valid field access on scene objects should pass");
}

#[test]
fn field_access_invalid_key_error() {
    let input = fixture_input("tests/fixtures/semantic/field_access_invalid_key_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidFieldAccessKey(_))),
        "Expected invalid field access key error",
    );
}

#[test]
fn field_access_on_non_struct_error() {
    let input = fixture_input("tests/fixtures/semantic/field_access_on_non_struct_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidFieldAccess(_))),
        "Expected invalid field access type error",
    );
}

#[test]
fn struct_literal_validation_valid_ok() {
    let input = fixture_input("tests/fixtures/semantic/struct_literal_validation_valid_ok");
    run_semantic(&input).expect("Valid struct literals should pass field validation");
}

#[test]
fn struct_literal_invalid_key_error() {
    let input = fixture_input("tests/fixtures/semantic/struct_literal_invalid_key_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidKey(_))),
        "Expected invalid key error for struct literal",
    );
}

#[test]
fn struct_literal_field_type_mismatch_error() {
    let input = fixture_input("tests/fixtures/semantic/struct_literal_field_type_mismatch_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::TypeMismatch { .. })),
        "Expected type mismatch error for struct literal field",
    );
}

#[test]
fn point3_and_color_struct_literals_ok() {
    let input = fixture_input("tests/fixtures/semantic/point3_and_color_struct_literals_ok");
    run_semantic(&input).expect("Point3 and Color literals should validate");
}

#[test]
fn point3_vec3_fields_must_be_float_error() {
    let input = fixture_input("tests/fixtures/semantic/point3_vec3_fields_must_be_float_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::TypeMismatch { .. })),
        "Expected type mismatch error for Point3/Vec3 field type",
    );
}

#[test]
fn material_variants_in_sphere_ok() {
    let input = fixture_input("tests/fixtures/semantic/material_variants_in_sphere_ok");
    run_semantic(&input)
        .expect("Lambertian/Metal/Dielectric should satisfy Material in Sphere");
}

#[test]
fn unknown_named_argument_errors() {
    let input = fixture_input("tests/fixtures/semantic/unknown_named_arg_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::UnknownArg(_))),
        "Expected unknown named argument error",
    );
}

#[test]
fn undeclared_variable_error() {
    let input = fixture_input("tests/fixtures/semantic/undeclared_variable_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::UndeclaredVariable(_))),
        "Expected undeclared variable error",
    );
}

#[test]
fn undefined_function_error() {
    let input = fixture_input("tests/fixtures/semantic/undefined_function_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::UndefinedFunction(_))),
        "Expected undefined function error",
    );
}

#[test]
fn wrong_arg_count_error() {
    let input = fixture_input("tests/fixtures/semantic/wrong_arg_count_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::WrongArgCount { .. })),
        "Expected wrong argument count error",
    );
}

#[test]
fn arg_type_mismatch_error() {
    let input = fixture_input("tests/fixtures/semantic/arg_type_mismatch_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::ArgTypeMismatch { .. })),
        "Expected argument type mismatch error",
    );
}

#[test]
fn duplicate_named_arg_error() {
    let input = fixture_input("tests/fixtures/semantic/duplicate_named_arg_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::DuplicateArg(_))),
        "Expected duplicate named argument error",
    );
}

#[test]
fn invalid_type_annotation_error() {
    let input = fixture_input("tests/fixtures/semantic/invalid_type_annotation_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidTypeAnnotation(_))),
        "Expected invalid type annotation error",
    );
}

#[test]
fn invalid_binary_assign_error() {
    let input = fixture_input("tests/fixtures/semantic/invalid_binary_assign_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidBinaryAssign { .. })),
        "Expected invalid binary assignment error",
    );
}

#[test]
fn invalid_http_endpoint_for_method_error() {
    let input = fixture_input("tests/fixtures/semantic/invalid_http_endpoint_for_method_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidEndpoint(_))),
        "Expected invalid endpoint for HTTP method error",
    );
}

#[test]
fn invalid_http_request_body_type_error() {
    let input = fixture_input("tests/fixtures/semantic/invalid_http_request_body_type_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::InvalidRequestBody(_))),
        "Expected invalid HTTP request body type error",
    );
}

#[test]
fn http_request_expects_struct_literal_error() {
    let input = fixture_input("tests/fixtures/semantic/http_request_expects_struct_literal_err");
    let errors = run_semantic(&input).expect_err("Semantic analysis should fail");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e.kind, SemanticErrorKind::ExpectedStructLiteral)),
        "Expected struct literal request body error",
    );
}
