//! HIR structurizer tests
//! Compares the structured HIR tree (SSA -> HIR) against a hand-built expected tree.
//!
//! Unlike the SSA IR tests, these don't compare against an `expected.txt` snapshot:
//! HIR nodes derive `Debug`/`PartialEq`, so we build the expected tree directly in
//! Rust (mirroring the constructor-function style used in `tests/parser.rs`) and
//! diff it with `colored_assert_debug`.

use razz_compiler::ast::{
    SpecificTypeKind, TypeKind,
    expression::{BinOpKind, EndpointKind, Literal, UnOpKind},
    statement::HTTPMethodKind,
};
use razz_compiler::compiler::{
    compiler::{Compiler, CompilerOutput, CompilerStage},
    error::CompilerError,
};
use razz_compiler::ir::Temp;
use razz_compiler::ir::hir::hir::{HIRBlock, HIRFunctionParam};
use razz_compiler::ir::hir::hir_expression::{HIRExpr, HIRFieldInit};
use razz_compiler::ir::hir::hir_statement::{HIRFunction, HIRProgram, HIRStmt};

#[cfg(test)]
mod common;
use common::{colored_assert_debug, load_fixture};

fn run_hir(input: &str) -> HIRProgram {
    let compiler = Compiler::new(CompilerStage::HIR);
    match compiler.compiles(input, None) {
        Ok(CompilerOutput::HIR(prog)) => prog,
        Ok(_) => panic!("Compiler flag mismatch"),
        Err(CompilerError::Lexer(errors)) => panic!("Unexpected lexer error: {:?}", errors),
        Err(CompilerError::Parser(errors)) => panic!("Unexpected parser error: {:?}", errors),
        Err(CompilerError::SemanticAnalysis(errors)) => {
            panic!("Unexpected semantic error: {:?}", errors)
        }
        Err(_) => panic!("Unexpected compiler stage error"),
    }
}

fn assert_hir_fixture(path: &str, expected: HIRProgram) {
    let (input, _) = load_fixture(path);
    let actual = run_hir(&input);
    colored_assert_debug(&actual, &expected);
}

// ============== Builders ==============

fn program(functions: Vec<HIRFunction>) -> HIRProgram {
    HIRProgram { functions }
}

fn func(name: &str, params: Vec<HIRFunctionParam>, return_ty: TypeKind, block: HIRBlock) -> HIRFunction {
    HIRFunction {
        name: name.to_string(),
        params,
        block,
        return_ty,
    }
}

fn param(name: &str, ty: TypeKind) -> HIRFunctionParam {
    HIRFunctionParam {
        name: name.to_string(),
        ty,
    }
}

fn t(id: u32, ty: TypeKind) -> Temp {
    Temp { id, ty }
}

// ---- statements ----

fn assign(target: Temp, expr: HIRExpr) -> HIRStmt {
    HIRStmt::Assign { target, expr }
}

fn while_stmt(cond: HIRExpr, block: HIRBlock) -> HIRStmt {
    HIRStmt::While { cond, block }
}

fn if_stmt(cond: HIRExpr, body: HIRBlock, else_body: HIRBlock) -> HIRStmt {
    HIRStmt::If { cond, body, else_body }
}

fn ret(value: HIRExpr) -> HIRStmt {
    HIRStmt::Return(value)
}

fn http_request(method: HTTPMethodKind, ep: EndpointKind, body: HIRExpr) -> HIRStmt {
    HIRStmt::HTTPRequest { method, ep, body }
}

// ---- expressions ----

fn bin(lhs: HIRExpr, op: BinOpKind, rhs: HIRExpr) -> HIRExpr {
    HIRExpr::BinOp {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }
}

fn un(op: UnOpKind, value: HIRExpr) -> HIRExpr {
    HIRExpr::UnOp {
        op,
        value: Box::new(value),
    }
}

fn if_expr(cond: HIRExpr, then: HIRExpr, else_: HIRExpr) -> HIRExpr {
    HIRExpr::If {
        cond: Box::new(cond),
        then: Box::new(then),
        else_: Box::new(else_),
    }
}

fn call(name: &str, args: Vec<HIRExpr>) -> HIRExpr {
    HIRExpr::FunctionCall {
        name: name.to_string(),
        args,
    }
}

fn field_access(obj: HIRExpr, key: &str) -> HIRExpr {
    HIRExpr::FieldAccess {
        obj: Box::new(obj),
        key: key.to_string(),
    }
}

fn http_get(ep: EndpointKind) -> HIRExpr {
    HIRExpr::HTTPRequest(ep)
}

fn field_store(obj: HIRExpr, key: &str, value: HIRExpr) -> HIRStmt {
    HIRStmt::FieldStore {
        obj,
        key: key.to_string(),
        value,
    }
}

fn struct_lit(ty: SpecificTypeKind, fields: Vec<HIRFieldInit>) -> HIRExpr {
    HIRExpr::StructLiteral { ty, fields }
}

fn field(name: &str, value: HIRExpr) -> HIRFieldInit {
    HIRFieldInit {
        name: name.to_string(),
        value,
    }
}

fn temp(id: u32, ty: TypeKind) -> HIRExpr {
    HIRExpr::Temp(t(id, ty))
}

fn int(v: i32) -> HIRExpr {
    HIRExpr::Const(Literal::Int(v))
}

fn float(v: f64) -> HIRExpr {
    HIRExpr::Const(Literal::Float(v))
}

fn null_lit() -> HIRExpr {
    HIRExpr::Const(Literal::Null)
}

fn bool_lit(v: bool) -> HIRExpr {
    HIRExpr::Const(Literal::Bool(v))
}

// ============== Tests ==============

/// `i` (loop counter) and `counter` (loop-carried value) each get a temp before the
/// while loop; the body increments both, then reassigns both loop-carried temps
/// before looping back.
#[test]
fn while_loop() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            assign(t(0, TypeKind::Int), int(0)),
            assign(t(2, TypeKind::Int), int(0)),
            while_stmt(
                bin(temp(0, TypeKind::Int), BinOpKind::Lt, int(3)),
                vec![
                    assign(t(3, TypeKind::Int), bin(temp(2, TypeKind::Int), BinOpKind::Add, int(1))),
                    assign(t(4, TypeKind::Int), bin(temp(0, TypeKind::Int), BinOpKind::Add, int(1))),
                    assign(t(0, TypeKind::Int), temp(4, TypeKind::Int)),
                    assign(t(2, TypeKind::Int), temp(3, TypeKind::Int)),
                ],
            ),
            ret(temp(2, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/while_loop", expected);
}

/// Plain if/else phi: exactly one `Assign { expr: HIRExpr::If }`, no leftover
/// `HIRStmt::If` (no side-effecting statements in either branch to preserve).
#[test]
fn if_else() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            assign(
                t(1, TypeKind::Int),
                if_expr(bin(int(1), BinOpKind::Gt, int(0)), int(1), int(2)),
            ),
            ret(temp(1, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/if_else", expected);
}

/// 3-way phi merge (if / else-if / else). The negated-condition and
/// misplaced-`Return` bug is fixed, but the structurizer still emits a
/// leftover `HIRStmt::If` that redundantly recomputes the same merge before
/// the real (correct) phi `Assign` + top-level `Return` -- this is the same
/// "unoptimized leftover branch" pattern documented elsewhere (e.g.
/// `if_fn_call_phi`), harmless but wasteful until a DCE pass removes it.
#[test]
fn else_if_chain() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            if_stmt(
                un(UnOpKind::Not, bin(int(1), BinOpKind::Gt, int(2))),
                vec![assign(
                    t(2, TypeKind::Int),
                    if_expr(bin(int(3), BinOpKind::Gt, int(2)), int(2), int(3)),
                )],
                vec![],
            ),
            assign(
                t(2, TypeKind::Int),
                if_expr(
                    bin(int(1), BinOpKind::Gt, int(2)),
                    int(1),
                    if_expr(bin(int(3), BinOpKind::Gt, int(2)), int(2), int(3)),
                ),
            ),
            ret(temp(2, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/else_if_chain", expected);
}

/// Same shape as `else_if_chain`: a 3-way phi merge (if / else-if / else) where
/// the negated-condition/misplaced-`Return` bug is fixed. What remains is the
/// same leftover-`HIRStmt::If` redundancy noted there -- a duplicate
/// recomputation of the else-if branch left before the real phi `Assign`.
#[test]
fn multiple_phis_merging_path() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            if_stmt(
                un(UnOpKind::Not, bin(int(1), BinOpKind::Gt, int(1))),
                vec![assign(
                    t(2, TypeKind::Int),
                    if_expr(bin(int(1), BinOpKind::Le, int(10)), int(5), int(8)),
                )],
                vec![],
            ),
            assign(
                t(2, TypeKind::Int),
                if_expr(
                    bin(int(1), BinOpKind::Gt, int(1)),
                    int(2),
                    if_expr(bin(int(1), BinOpKind::Le, int(10)), int(5), int(8)),
                ),
            ),
            ret(temp(2, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/multiple_phis_merging_path", expected);
}

/// If branch calls `add(a: x, b: 1)` for its side effect (kept as a leftover
/// `HIRStmt::If` since the call isn't pure from the structurizer's perspective),
/// then a single phi assign recomputes the same call to produce the merged value.
#[test]
fn if_fn_call_phi() {
    let expected = program(vec![
        func(
            "add",
            vec![param("a", TypeKind::Int), param("b", TypeKind::Int)],
            TypeKind::Int,
            vec![
                assign(
                    t(2, TypeKind::Int),
                    bin(temp(0, TypeKind::Int), BinOpKind::Add, temp(1, TypeKind::Int)),
                ),
                ret(temp(2, TypeKind::Int)),
            ],
        ),
        func(
            "main",
            vec![],
            TypeKind::Int,
            vec![
                if_stmt(
                    bin(int(1), BinOpKind::Gt, int(0)),
                    vec![assign(t(4, TypeKind::Int), call("add", vec![int(0), int(1)]))],
                    vec![],
                ),
                assign(
                    t(5, TypeKind::Int),
                    if_expr(
                        bin(int(1), BinOpKind::Gt, int(0)),
                        call("add", vec![int(0), int(1)]),
                        int(2),
                    ),
                ),
                ret(temp(5, TypeKind::Int)),
            ],
        ),
    ]);
    assert_hir_fixture("tests/fixtures/hir/if_fn_call_phi", expected);
}

/// POST inside both if/else branches; the function returns null so there's no
/// phi to resolve - just the `HTTPRequest` side effects preserved per-branch.
#[test]
fn http_in_if() {
    let vec3_ty = TypeKind::SpecificType(SpecificTypeKind::Vec3);
    let color_ty = TypeKind::SpecificType(SpecificTypeKind::Color);
    let lambertian_ty = TypeKind::SpecificType(SpecificTypeKind::Lambertian);
    let sphere_ty = TypeKind::SpecificType(SpecificTypeKind::Sphere);

    let sphere_literal = |vec3_temp: u32, _color_temp: u32, lambertian_temp: u32, radius: f64| {
        struct_lit(
            SpecificTypeKind::Sphere,
            vec![
                field("coord", temp(vec3_temp, vec3_ty)),
                field("radius", float(radius)),
                field("material", temp(lambertian_temp, lambertian_ty)),
            ],
        )
    };

    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Null,
        vec![
            if_stmt(
                bin(int(1), BinOpKind::Gt, int(0)),
                vec![
                    assign(
                        t(2, vec3_ty),
                        struct_lit(
                            SpecificTypeKind::Vec3,
                            vec![
                                field("x", float(1.0)),
                                field("y", float(2.0)),
                                field("z", float(3.0)),
                            ],
                        ),
                    ),
                    assign(
                        t(4, color_ty),
                        struct_lit(
                            SpecificTypeKind::Color,
                            vec![field("r", int(10)), field("g", int(20)), field("b", int(30))],
                        ),
                    ),
                    assign(
                        t(3, lambertian_ty),
                        struct_lit(
                            SpecificTypeKind::Lambertian,
                            vec![field("albedo", temp(4, color_ty))],
                        ),
                    ),
                    assign(t(1, sphere_ty), sphere_literal(2, 4, 3, 0.5)),
                    http_request(HTTPMethodKind::Post, EndpointKind::Hittable, temp(1, sphere_ty)),
                ],
                vec![
                    assign(
                        t(6, vec3_ty),
                        struct_lit(
                            SpecificTypeKind::Vec3,
                            vec![
                                field("x", float(0.0)),
                                field("y", float(0.0)),
                                field("z", float(0.0)),
                            ],
                        ),
                    ),
                    assign(
                        t(8, color_ty),
                        struct_lit(
                            SpecificTypeKind::Color,
                            vec![field("r", int(1)), field("g", int(2)), field("b", int(3))],
                        ),
                    ),
                    assign(
                        t(7, lambertian_ty),
                        struct_lit(
                            SpecificTypeKind::Lambertian,
                            vec![field("albedo", temp(8, color_ty))],
                        ),
                    ),
                    assign(t(5, sphere_ty), sphere_literal(6, 8, 7, 1.0)),
                    http_request(HTTPMethodKind::Post, EndpointKind::Hittable, temp(5, sphere_ty)),
                ],
            ),
            ret(null_lit()),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/http_in_if", expected);
}

/// if/else nested inside a for (desugared to while) loop. The phi assign for `x`
/// appears before the loop counter increment and loop-carried-value update.
#[test]
fn nested_loop_if() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            assign(t(0, TypeKind::Int), int(0)),
            assign(t(5, TypeKind::Int), int(0)),
            while_stmt(
                bin(temp(0, TypeKind::Int), BinOpKind::Lt, int(3)),
                vec![
                    assign(
                        t(6, TypeKind::Int),
                        if_expr(bin(temp(0, TypeKind::Int), BinOpKind::Gt, int(1)), int(1), int(2)),
                    ),
                    assign(t(4, TypeKind::Int), bin(temp(0, TypeKind::Int), BinOpKind::Add, int(1))),
                    assign(t(0, TypeKind::Int), temp(4, TypeKind::Int)),
                    assign(t(5, TypeKind::Int), temp(6, TypeKind::Int)),
                ],
            ),
            ret(temp(5, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/nested_loop_if", expected);
}

/// if nested inside if nested inside a for (desugared to while) loop. One phi
/// assign per loop iteration, with a nested `HIRExpr::If` in the then branch;
/// a leftover (unoptimized, DCE pending) `HIRStmt::If` with an empty else_body
/// precedes it since there are no side-effecting statements to preserve.
#[test]
fn deep_nest() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            assign(t(0, TypeKind::Int), int(0)),
            assign(t(7, TypeKind::Int), int(0)),
            while_stmt(
                bin(temp(0, TypeKind::Int), BinOpKind::Lt, int(3)),
                vec![
                    if_stmt(
                        bin(temp(0, TypeKind::Int), BinOpKind::Gt, int(1)),
                        vec![assign(
                            t(9, TypeKind::Int),
                            if_expr(bin(temp(0, TypeKind::Int), BinOpKind::Gt, int(2)), int(1), int(2)),
                        )],
                        vec![],
                    ),
                    assign(
                        t(8, TypeKind::Int),
                        if_expr(
                            bin(temp(0, TypeKind::Int), BinOpKind::Gt, int(1)),
                            if_expr(bin(temp(0, TypeKind::Int), BinOpKind::Gt, int(2)), int(1), int(2)),
                            int(3),
                        ),
                    ),
                    assign(t(6, TypeKind::Int), bin(temp(0, TypeKind::Int), BinOpKind::Add, int(1))),
                    assign(t(0, TypeKind::Int), temp(6, TypeKind::Int)),
                    assign(t(7, TypeKind::Int), temp(8, TypeKind::Int)),
                ],
            ),
            ret(temp(7, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/deep_nest", expected);
}

/// Straight-line arithmetic, no control flow: `x = 1; y = x + 2; return y;`.
#[test]
fn simple_binop() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            assign(t(0, TypeKind::Int), bin(int(1), BinOpKind::Add, int(2))),
            ret(temp(0, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/simple_binop", expected);
}

/// Unary minus feeding into a function call: `x = -1; y = add(a: x, b: 2);`.
#[test]
fn call_and_unary() {
    let expected = program(vec![
        func(
            "add",
            vec![param("a", TypeKind::Int), param("b", TypeKind::Int)],
            TypeKind::Int,
            vec![
                assign(
                    t(2, TypeKind::Int),
                    bin(temp(0, TypeKind::Int), BinOpKind::Add, temp(1, TypeKind::Int)),
                ),
                ret(temp(2, TypeKind::Int)),
            ],
        ),
        func(
            "main",
            vec![],
            TypeKind::Int,
            vec![
                assign(t(3, TypeKind::Int), un(UnOpKind::Minus, int(1))),
                assign(t(4, TypeKind::Int), call("add", vec![temp(3, TypeKind::Int), int(2)])),
                ret(temp(4, TypeKind::Int)),
            ],
        ),
    ]);
    assert_hir_fixture("tests/fixtures/hir/call_and_unary", expected);
}

/// GET into a struct-typed temp, then a plain field store followed by a
/// compound field store (`cam->vfov += 10`, desugared to a `FieldAccess` +
/// `BinOp::Add` + `FieldStore`).
#[test]
fn field_compound_assign() {
    let camera_ty = TypeKind::SpecificType(SpecificTypeKind::Camera);
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Null,
        vec![
            assign(t(0, camera_ty), http_get(EndpointKind::Camera)),
            field_store(temp(0, camera_ty), "vfov", int(90)),
            assign(t(1, TypeKind::Int), field_access(temp(0, camera_ty), "vfov")),
            assign(t(2, TypeKind::Int), bin(temp(1, TypeKind::Int), BinOpKind::Add, int(10))),
            field_store(temp(0, camera_ty), "vfov", temp(2, TypeKind::Int)),
            ret(null_lit()),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/field_compound_assign", expected);
}

/// Field store into a nested struct field: `cam->lookfrom->x = 4.0` reads
/// `cam->lookfrom` into a temp, then stores into that temp's `x` field.
#[test]
fn nested_field_assign() {
    let camera_ty = TypeKind::SpecificType(SpecificTypeKind::Camera);
    let vec3_ty = TypeKind::SpecificType(SpecificTypeKind::Vec3);
    let point3_ty = TypeKind::SpecificType(SpecificTypeKind::Point3);
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Null,
        vec![
            assign(t(0, camera_ty), http_get(EndpointKind::Camera)),
            assign(
                t(1, vec3_ty),
                struct_lit(
                    SpecificTypeKind::Vec3,
                    vec![field("x", float(1.0)), field("y", float(2.0)), field("z", float(3.0))],
                ),
            ),
            field_store(temp(0, camera_ty), "lookfrom", temp(1, vec3_ty)),
            assign(t(2, point3_ty), field_access(temp(0, camera_ty), "lookfrom")),
            field_store(temp(2, point3_ty), "x", float(4.0)),
            ret(null_lit()),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/nested_field_assign", expected);
}

/// If inside if, both fully if/else (no bare if-without-else, which hits a
/// separate known bug - see module docs). One phi assign per level: the outer
/// false-branch keeps a leftover `HIRStmt::If` computing the inner phi for its
/// (unreachable at runtime, but still emitted) side effect, then the top-level
/// phi assign recomputes the same nested `HIRExpr::If`, followed by a single
/// `Return`.
#[test]
fn nested_if() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            if_stmt(
                bool_lit(false),
                vec![assign(
                    t(1, TypeKind::Int),
                    if_expr(bool_lit(true), int(1), int(3)),
                )],
                vec![],
            ),
            assign(
                t(0, TypeKind::Int),
                if_expr(bool_lit(false), if_expr(bool_lit(true), int(1), int(3)), int(2)),
            ),
            ret(temp(0, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/nested_if", expected);
}

/// Mixed HTTP GET/field-access/struct-literal/logical-operator/if-else
/// expression, exercising `UnOp::Not`, `BinOp::And`/`BinOp::Or` fully inlined
/// directly into the `If`'s `cond` (compound conditions are no longer hoisted
/// into standalone temps, consistent with `while_compound_cond`), alongside a
/// leftover `HIRStmt::If` (fn call in body) preceding the phi `Assign`.
#[test]
fn advanced_exprs() {
    let camera_ty = TypeKind::SpecificType(SpecificTypeKind::Camera);
    let color_ty = TypeKind::SpecificType(SpecificTypeKind::Color);
    let expected = program(vec![
        func(
            "add",
            vec![param("a", TypeKind::Int), param("b", TypeKind::Int)],
            TypeKind::Int,
            vec![
                assign(
                    t(2, TypeKind::Int),
                    bin(temp(0, TypeKind::Int), BinOpKind::Add, temp(1, TypeKind::Int)),
                ),
                ret(temp(2, TypeKind::Int)),
            ],
        ),
        func(
            "main",
            vec![],
            TypeKind::Int,
            vec![
                assign(t(3, camera_ty), http_get(EndpointKind::Camera)),
                field_store(temp(3, camera_ty), "vfov", float(45.5)),
                assign(t(5, TypeKind::Int), call("add", vec![int(1), int(2)])),
                assign(
                    t(4, color_ty),
                    struct_lit(
                        SpecificTypeKind::Color,
                        vec![
                            field("r", temp(5, TypeKind::Int)),
                            field("g", int(3)),
                            field("b", int(4)),
                        ],
                    ),
                ),
                if_stmt(
                    bin(
                        un(UnOpKind::Not, bin(int(1), BinOpKind::Lt, int(2))),
                        BinOpKind::Or,
                        bin(bin(int(3), BinOpKind::Le, int(4)), BinOpKind::And, bool_lit(true)),
                    ),
                    vec![
                        assign(t(12, TypeKind::Int), field_access(temp(4, color_ty), "r")),
                        assign(
                            t(11, TypeKind::Int),
                            call("add", vec![temp(12, TypeKind::Int), int(5)]),
                        ),
                    ],
                    vec![assign(t(13, TypeKind::Int), un(UnOpKind::Minus, int(1)))],
                ),
                assign(
                    t(14, TypeKind::Int),
                    if_expr(
                        bin(
                            un(UnOpKind::Not, bin(int(1), BinOpKind::Lt, int(2))),
                            BinOpKind::Or,
                            bin(bin(int(3), BinOpKind::Le, int(4)), BinOpKind::And, bool_lit(true)),
                        ),
                        call("add", vec![temp(12, TypeKind::Int), int(5)]),
                        un(UnOpKind::Minus, int(1)),
                    ),
                ),
                ret(temp(14, TypeKind::Int)),
            ],
        ),
    ]);
    assert_hir_fixture("tests/fixtures/hir/advanced_exprs", expected);
}

/// A full HTTP "scene setup" sequence: GET, a plain field store, PATCH with a
/// struct literal reading back the field, POST with deeply nested struct
/// literals, PUT, and a final PATCH - all void, no phi involved.
#[test]
fn advanced_http_scene() {
    let camera_ty = TypeKind::SpecificType(SpecificTypeKind::Camera);
    let vec3_ty = TypeKind::SpecificType(SpecificTypeKind::Vec3);
    let color_ty = TypeKind::SpecificType(SpecificTypeKind::Color);
    let lambertian_ty = TypeKind::SpecificType(SpecificTypeKind::Lambertian);
    let sphere_ty = TypeKind::SpecificType(SpecificTypeKind::Sphere);
    let background_ty = TypeKind::SpecificType(SpecificTypeKind::Background);
    let output_ty = TypeKind::SpecificType(SpecificTypeKind::Output);

    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Null,
        vec![
            assign(t(0, camera_ty), http_get(EndpointKind::Camera)),
            field_store(temp(0, camera_ty), "vfov", float(60.5)),
            assign(t(2, TypeKind::Float), field_access(temp(0, camera_ty), "vfov")),
            assign(
                t(1, camera_ty),
                struct_lit(SpecificTypeKind::Camera, vec![field("vfov", temp(2, TypeKind::Float))]),
            ),
            http_request(HTTPMethodKind::Patch, EndpointKind::Camera, temp(1, camera_ty)),
            assign(
                t(4, vec3_ty),
                struct_lit(
                    SpecificTypeKind::Vec3,
                    vec![field("x", float(1.0)), field("y", float(2.0)), field("z", float(3.0))],
                ),
            ),
            assign(
                t(6, color_ty),
                struct_lit(
                    SpecificTypeKind::Color,
                    vec![field("r", int(10)), field("g", int(20)), field("b", int(30))],
                ),
            ),
            assign(
                t(5, lambertian_ty),
                struct_lit(SpecificTypeKind::Lambertian, vec![field("albedo", temp(6, color_ty))]),
            ),
            assign(
                t(3, sphere_ty),
                struct_lit(
                    SpecificTypeKind::Sphere,
                    vec![
                        field("coord", temp(4, vec3_ty)),
                        field("radius", float(0.5)),
                        field("material", temp(5, lambertian_ty)),
                    ],
                ),
            ),
            http_request(HTTPMethodKind::Post, EndpointKind::Hittable, temp(3, sphere_ty)),
            assign(
                t(8, vec3_ty),
                struct_lit(
                    SpecificTypeKind::Vec3,
                    vec![field("x", float(0.1)), field("y", float(0.2)), field("z", float(0.3))],
                ),
            ),
            assign(
                t(9, vec3_ty),
                struct_lit(
                    SpecificTypeKind::Vec3,
                    vec![field("x", float(0.0)), field("y", float(0.0)), field("z", float(0.0))],
                ),
            ),
            assign(
                t(7, background_ty),
                struct_lit(
                    SpecificTypeKind::Background,
                    vec![field("top", temp(8, vec3_ty)), field("bottom", temp(9, vec3_ty))],
                ),
            ),
            http_request(HTTPMethodKind::Put, EndpointKind::Background, temp(7, background_ty)),
            assign(
                t(10, output_ty),
                struct_lit(
                    SpecificTypeKind::Output,
                    vec![field("file", HIRExpr::Const(Literal::String("scene.ppm".to_string())))],
                ),
            ),
            http_request(HTTPMethodKind::Patch, EndpointKind::Output, temp(10, output_ty)),
            ret(null_lit()),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/advanced_http_scene", expected);
}

/// `x: int = 1; return x;` with an explicit type annotation - straight-line,
/// constant-folds all the way through to `return 1;` (no control flow to force
/// a temp/phi to survive).
#[test]
fn assign_type_annotation() {
    let expected = program(vec![func("main", vec![], TypeKind::Int, vec![ret(int(1))])]);
    assert_hir_fixture("tests/fixtures/hir/assign_type_annotation", expected);
}

/// `if true { dummy = 1; } return null;` with `dummy` never used after: SSA
/// laziness (see module docs) prunes the unused assign, and since the branch
/// then has no observable effect at all, the whole `if` construct is elided too
/// - the function body collapses to just the trailing `Return`.
#[test]
fn if_without_else() {
    let expected = program(vec![func("main", vec![], TypeKind::Null, vec![ret(null_lit())])]);
    assert_hir_fixture("tests/fixtures/hir/if_without_else", expected);
}

#[test]
fn nested_while() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Null,
        vec![
            while_stmt(bool_lit(false), vec![while_stmt(bool_lit(false), vec![])]),
            ret(null_lit()),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/nested_while", expected);
}

#[test]
fn nested_for() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Null,
        vec![
            while_stmt(bool_lit(false), vec![while_stmt(bool_lit(false), vec![])]),
            ret(null_lit()),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/nested_for", expected);
}

#[test]
fn for_no_condition() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Null,
        vec![while_stmt(bool_lit(true), vec![]), ret(null_lit())],
    )]);
    assert_hir_fixture("tests/fixtures/hir/for_no_condition", expected);
}

/// String concatenation: `return "ra" + "zz";` - straight-line `BinOp` over
/// `String` literals.
#[test]
fn string_concat() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::String,
        vec![
            assign(
                t(0, TypeKind::String),
                bin(
                    HIRExpr::Const(Literal::String("ra".to_string())),
                    BinOpKind::Add,
                    HIRExpr::Const(Literal::String("zz".to_string())),
                ),
            ),
            ret(temp(0, TypeKind::String)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/string_concat", expected);
}

/// Two independently-merged variables (`a`, `b`) assigned in both branches of
/// the same if/else: each gets its own phi `Assign`, and both feed into a
/// single downstream `BinOp`.
#[test]
fn two_vars_phi_same_branch() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            assign(
                t(1, TypeKind::Int),
                if_expr(bin(int(1), BinOpKind::Gt, int(0)), int(1), int(3)),
            ),
            assign(
                t(2, TypeKind::Int),
                if_expr(bin(int(1), BinOpKind::Gt, int(0)), int(2), int(4)),
            ),
            assign(
                t(3, TypeKind::Int),
                bin(temp(1, TypeKind::Int), BinOpKind::Add, temp(2, TypeKind::Int)),
            ),
            ret(temp(3, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/two_vars_phi_same_branch", expected);
}

/// `for` loop accumulating into `sum` (`sum += i`) alongside the implicit loop
/// counter `i` - two independent loop-carried temps, neither of which is the
/// bare counter-only pattern already covered by `while_loop`/`nested_loop_if`.
#[test]
fn for_loop_multi_carry() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            assign(t(0, TypeKind::Int), int(0)),
            assign(t(2, TypeKind::Int), int(0)),
            while_stmt(
                bin(temp(0, TypeKind::Int), BinOpKind::Lt, int(3)),
                vec![
                    assign(t(3, TypeKind::Int), bin(temp(2, TypeKind::Int), BinOpKind::Add, temp(0, TypeKind::Int))),
                    assign(t(4, TypeKind::Int), bin(temp(0, TypeKind::Int), BinOpKind::Add, int(1))),
                    assign(t(0, TypeKind::Int), temp(4, TypeKind::Int)),
                    assign(t(2, TypeKind::Int), temp(3, TypeKind::Int)),
                ],
            ),
            ret(temp(2, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/for_loop_multi_carry", expected);
}

/// `bool`-typed phi merge: `flag = false; if 1 > 0 { flag = true; } else {
/// flag = false; } return flag;` - the phi assign target/branches are `Bool`,
/// not `Int` like every other phi test so far.
#[test]
fn bool_var_phi() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Bool,
        vec![
            assign(
                t(1, TypeKind::Bool),
                if_expr(bin(int(1), BinOpKind::Gt, int(0)), bool_lit(true), bool_lit(false)),
            ),
            ret(temp(1, TypeKind::Bool)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/bool_var_phi", expected);
}

/// Returning a function call's result directly, with no intermediate named
/// variable: `return add(a: 1, b: 2);`.
#[test]
fn return_call_direct() {
    let expected = program(vec![
        func(
            "add",
            vec![param("a", TypeKind::Int), param("b", TypeKind::Int)],
            TypeKind::Int,
            vec![
                assign(
                    t(2, TypeKind::Int),
                    bin(temp(0, TypeKind::Int), BinOpKind::Add, temp(1, TypeKind::Int)),
                ),
                ret(temp(2, TypeKind::Int)),
            ],
        ),
        func(
            "main",
            vec![],
            TypeKind::Int,
            vec![
                assign(t(3, TypeKind::Int), call("add", vec![int(1), int(2)])),
                ret(temp(3, TypeKind::Int)),
            ],
        ),
    ]);
    assert_hir_fixture("tests/fixtures/hir/return_call_direct", expected);
}

/// Float arithmetic with unary minus: `x = 1.5; y = -x + 2.5; return y;`.
#[test]
fn float_ops() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Float,
        vec![
            assign(t(0, TypeKind::Float), un(UnOpKind::Minus, float(1.5))),
            assign(t(1, TypeKind::Float), bin(temp(0, TypeKind::Float), BinOpKind::Add, float(2.5))),
            ret(temp(1, TypeKind::Float)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/float_ops", expected);
}

/// Field store/access on a locally-constructed struct literal (no HTTP GET
/// involved, unlike `field_compound_assign`/`nested_field_assign`): `v = Vec3 {
/// .. }; v->x = 9.0; return v->x;`.
#[test]
fn struct_field_local() {
    let vec3_ty = TypeKind::SpecificType(SpecificTypeKind::Vec3);
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Float,
        vec![
            assign(
                t(0, vec3_ty),
                struct_lit(
                    SpecificTypeKind::Vec3,
                    vec![field("x", float(1.0)), field("y", float(2.0)), field("z", float(3.0))],
                ),
            ),
            field_store(temp(0, vec3_ty), "x", float(9.0)),
            assign(t(1, TypeKind::Float), field_access(temp(0, vec3_ty), "x")),
            ret(temp(1, TypeKind::Float)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/struct_field_local", expected);
}

#[test]
fn both_branches_return() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![if_stmt(
            bin(int(1), BinOpKind::Gt, int(0)),
            vec![ret(int(1))],
            vec![ret(int(2))],
        )],
    )]);
    assert_hir_fixture("tests/fixtures/hir/both_branches_return", expected);
}

#[test]
fn while_compound_cond() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Int,
        vec![
            assign(t(0, TypeKind::Int), int(0)),
            assign(t(2, TypeKind::Int), int(10)),
            while_stmt(
                bin(
                    bin(temp(0, TypeKind::Int), BinOpKind::Lt, int(5)),
                    BinOpKind::And,
                    bin(temp(2, TypeKind::Int), BinOpKind::Gt, int(0)),
                ),
                vec![
                    assign(t(5, TypeKind::Int), bin(temp(0, TypeKind::Int), BinOpKind::Add, int(1))),
                    assign(t(6, TypeKind::Int), bin(temp(2, TypeKind::Int), BinOpKind::Sub, int(1))),
                    assign(t(0, TypeKind::Int), temp(5, TypeKind::Int)),
                    assign(t(2, TypeKind::Int), temp(6, TypeKind::Int)),
                ],
            ),
            ret(temp(0, TypeKind::Int)),
        ],
    )]);
    assert_hir_fixture("tests/fixtures/hir/while_compound_cond", expected);
}
