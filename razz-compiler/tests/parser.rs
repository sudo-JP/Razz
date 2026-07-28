//! Parser tests
//! Most of the tests are written by AI, I code review over it 

use std::collections::HashSet;

use razz_compiler::ast::{
    expression::{Arg, BinOpKind, EndpointKind, Expr, ExprKind, Literal, StructField, UnOpKind}, statement::{
        Block, CompoundOpKind, ElseIf, FnDecl, HTTPMethodKind, Param, Stmt, StmtKind,
    }, traversal::{walk_expr, walk_fn_decl, walk_stmt, ASTWalkable}, NodeId, Program, Spanned, SpecificTypeKind, TypeKind
};
use razz_compiler::common::{Position, Span};
use razz_compiler::compiler::{
    compiler::{Compiler, CompilerOutput, CompilerStage},
    error::CompilerError,
};
use razz_compiler::parser::error::ParserError;

#[cfg(test)]
mod common;
use common::{colored_assert_debug, load_fixture};

fn run_parser(input: &str) -> Result<Program, Vec<ParserError>> {
    let compiler = Compiler::new(CompilerStage::Parser);
    match compiler.compiles(input, None) {
        Ok(CompilerOutput::Parser(p)) => Ok(p),
        Ok(_) => panic!("Compiler flag mismatch"),
        Err(CompilerError::Parser(e)) => Err(e),
        Err(_) => panic!("Lexer error"),
    }
}

fn zero_span() -> Span {
    Span {
        start: Position { line: 0, col: 0 },
        end: Position { line: 0, col: 0 },
    }
}

fn sp<T>(node: T) -> Spanned<T> {
    Spanned {
        node,
        span: zero_span(),
    }
}

fn expr(kind: ExprKind) -> Expr {
    Expr {
        id: 0,
        kind,
        span: zero_span(),
    }
}

fn stmt(kind: StmtKind) -> Stmt {
    Stmt {
        id: 0,
        kind,
        span: zero_span(),
    }
}

fn block(stmts: Vec<Stmt>) -> Block {
    Block {
        id: 0,
        stmts,
        span: zero_span(),
    }
}

fn func(name: &str, params: Vec<Param>, return_type: TypeKind, body: Vec<Stmt>) -> Spanned<FnDecl> {
    sp(FnDecl {
        id: 0,
        name: sp(name.to_string()),
        params,
        return_type: sp(return_type),
        body: block(body),
    })
}

fn param(name: &str, ty: TypeKind) -> Param {
    Param {
        name: sp(name.to_string()),
        ty: sp(ty),
        id: 0,
    }
}

fn arg(name: &str, value: Expr) -> Arg {
    Arg {
        name: sp(name.to_string()),
        expr: value,
    }
}

fn field(key: &str, value: Expr) -> StructField {
    StructField {
        key: sp(key.to_string()),
        value,
    }
}

fn else_if(cond: Expr, body: Vec<Stmt>) -> ElseIf {
    ElseIf {
        id: 0,
        span: zero_span(),
        cond,
        body: block(body),
    }
}

fn ident(name: &str) -> Expr {
    expr(ExprKind::Ident(name.to_string()))
}

fn int_lit(value: i32) -> Expr {
    expr(ExprKind::Constant(Literal::Int(value)))
}

fn float_lit(value: f64) -> Expr {
    expr(ExprKind::Constant(Literal::Float(value)))
}

fn bool_lit(value: bool) -> Expr {
    expr(ExprKind::Constant(Literal::Bool(value)))
}

fn bin(lhs: Expr, op: BinOpKind, rhs: Expr) -> Expr {
    expr(ExprKind::BinOp {
        lhs: Box::new(lhs),
        op: sp(op),
        rhs: Box::new(rhs),
    })
}

fn un(op: UnOpKind, value: Expr) -> Expr {
    expr(ExprKind::UnOp {
        op: sp(op),
        value: Box::new(value),
    })
}

fn call(name: &str, args: Vec<Arg>) -> Expr {
    expr(ExprKind::FunctionCall {
        name: sp(name.to_string()),
        args,
    })
}

fn access(obj: Expr, key: &str) -> Expr {
    expr(ExprKind::FieldAccess {
        obj: Box::new(obj),
        key: sp(key.to_string()),
    })
}

fn struct_lit(ty: SpecificTypeKind, fields: Vec<StructField>) -> Expr {
    expr(ExprKind::StructLiteral {
        ty: sp(ty),
        fields,
    })
}

fn get(endpoint: EndpointKind) -> Expr {
    expr(ExprKind::HTTPRequest(sp(endpoint)))
}

fn assign(name: &str, type_ann: Option<TypeKind>, value: Expr) -> Stmt {
    stmt(StmtKind::Assign {
        target: ident(name),
        type_ann: type_ann.map(sp),
        expr: value,
    })
}

fn compound_assign(name: &str, op: CompoundOpKind, value: Expr) -> Stmt {
    stmt(StmtKind::CompoundAssign {
        target: ident(name),
        op: sp(op),
        expr: value,
    })
}

fn assign_obj(target: Expr, value: Expr) -> Stmt {
    stmt(StmtKind::Assign {
        target,
        type_ann: None,
        expr: value,
    })
}

fn compound_assign_obj(target: Expr, op: CompoundOpKind, value: Expr) -> Stmt {
    stmt(StmtKind::CompoundAssign {
        target,
        op: sp(op),
        expr: value,
    })
}

fn while_stmt(cond: Expr, body: Vec<Stmt>) -> Stmt {
    stmt(StmtKind::While {
        cond,
        body: block(body),
    })
}

fn for_stmt(decl: Option<Stmt>, cond: Option<Expr>, update: Vec<Stmt>, body: Vec<Stmt>) -> Stmt {
    stmt(StmtKind::For {
        decl: decl.map(Box::new),
        cond,
        update,
        body: block(body),
    })
}

fn if_stmt(cond: Expr, body: Vec<Stmt>, else_ifs: Vec<ElseIf>, else_body: Option<Vec<Stmt>>) -> Stmt {
    stmt(StmtKind::If {
        cond,
        body: block(body),
        else_ifs,
        else_body: else_body.map(block),
    })
}

fn return_stmt(value: Expr) -> Stmt {
    stmt(StmtKind::Return(value))
}

fn http_stmt(method: HTTPMethodKind, endpoint: EndpointKind, body: Expr) -> Stmt {
    stmt(StmtKind::HTTPRequest {
        method: sp(method),
        endpoint: sp(endpoint),
        body,
    })
}

fn expr_stmt(value: Expr) -> Stmt {
    stmt(StmtKind::Expr(value))
}

fn program(funcs: Vec<Spanned<FnDecl>>) -> Program {
    Program { id: 0, funcs }
}

#[derive(Default)]
struct IdCollector {
    ids: HashSet<NodeId>,
}

impl IdCollector {
    fn insert(&mut self, id: NodeId, what: &str) {
        assert!(self.ids.insert(id), "duplicate {} NodeId {}", what, id);
    }

    fn collect_block(&mut self, block: &Block) {
        self.insert(block.id, "Block");
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
    }
}

impl ASTWalkable for IdCollector {
    fn visit_expr(&mut self, expr: &Expr) {
        self.insert(expr.id, "Expr");
        walk_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &Stmt) {
        self.insert(stmt.id, "Stmt");
        walk_stmt(self, stmt);
    }

    fn visit_if(
        &mut self,
        stmt: &Stmt,
        cond: &Expr,
        body: &Block,
        else_ifs: &[ElseIf],
        else_body: &Option<Block>,
    ) {
        let _ = stmt;
        self.visit_expr(cond);
        self.collect_block(body);
        for elif in else_ifs {
            self.insert(elif.id, "ElseIf");
            self.visit_expr(&elif.cond);
            self.collect_block(&elif.body);
        }
        if let Some(b) = else_body {
            self.collect_block(b);
        }
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        for param in &fn_decl.params {
            self.insert(param.id, "Param");
        }
        walk_fn_decl(self, fn_decl);
    }
}

fn assert_distinct_node_ids(program: &Program) {
    let mut collector = IdCollector::default();
    collector.insert(program.id, "Program");
    for func in &program.funcs {
        collector.insert(func.node.id, "FnDecl");
        collector.collect_block(&func.node.body);
    }
}

fn strip_ids_and_spans_expr(expr: &mut Expr) {
    expr.id = 0;
    expr.span = zero_span();
    match &mut expr.kind {
        ExprKind::BinOp { lhs, op, rhs } => {
            op.span = zero_span();
            strip_ids_and_spans_expr(lhs);
            strip_ids_and_spans_expr(rhs);
        }
        ExprKind::UnOp { op, value } => {
            op.span = zero_span();
            strip_ids_and_spans_expr(value);
        }
        ExprKind::FunctionCall { name, args } => {
            name.span = zero_span();
            for a in args {
                a.name.span = zero_span();
                strip_ids_and_spans_expr(&mut a.expr);
            }
        }
        ExprKind::FieldAccess { obj, key } => {
            key.span = zero_span();
            strip_ids_and_spans_expr(obj);
        }
        ExprKind::StructLiteral { ty, fields } => {
            ty.span = zero_span();
            for f in fields {
                f.key.span = zero_span();
                strip_ids_and_spans_expr(&mut f.value);
            }
        }
        ExprKind::HTTPRequest(endpoint) => {
            endpoint.span = zero_span();
        }
        ExprKind::Constant(_) | ExprKind::Ident(_) => {}
    }
}

fn strip_ids_and_spans_stmt(stmt: &mut Stmt) {
    stmt.id = 0;
    stmt.span = zero_span();
    match &mut stmt.kind {
        StmtKind::Assign {
            target,
            type_ann,
            expr,
        } => {
            strip_ids_and_spans_expr(target);
            if let Some(ty) = type_ann {
                ty.span = zero_span();
            }
            strip_ids_and_spans_expr(expr);
        }
        StmtKind::CompoundAssign { target, op, expr } => {
            strip_ids_and_spans_expr(target);
            op.span = zero_span();
            strip_ids_and_spans_expr(expr);
        }
        StmtKind::While { cond, body } => {
            strip_ids_and_spans_expr(cond);
            strip_ids_and_spans_block(body);
        }
        StmtKind::For {
            decl,
            cond,
            update,
            body,
        } => {
            if let Some(d) = decl {
                strip_ids_and_spans_stmt(d);
            }
            if let Some(c) = cond {
                strip_ids_and_spans_expr(c);
            }
            for u in update {
                strip_ids_and_spans_stmt(u);
            }
            strip_ids_and_spans_block(body);
        }
        StmtKind::If {
            cond,
            body,
            else_ifs,
            else_body,
        } => {
            strip_ids_and_spans_expr(cond);
            strip_ids_and_spans_block(body);
            for eif in else_ifs {
                eif.id = 0;
                eif.span = zero_span();
                strip_ids_and_spans_expr(&mut eif.cond);
                strip_ids_and_spans_block(&mut eif.body);
            }
            if let Some(else_blk) = else_body {
                strip_ids_and_spans_block(else_blk);
            }
        }
        StmtKind::Return(expr) => strip_ids_and_spans_expr(expr),
        StmtKind::HTTPRequest {
            method,
            endpoint,
            body,
        } => {
            method.span = zero_span();
            endpoint.span = zero_span();
            strip_ids_and_spans_expr(body);
        }
        StmtKind::Expr(expr) => strip_ids_and_spans_expr(expr),
    }
}

fn strip_ids_and_spans_block(block: &mut Block) {
    block.id = 0;
    block.span = zero_span();
    for stmt in &mut block.stmts {
        strip_ids_and_spans_stmt(stmt);
    }
}

fn strip_ids_and_spans_program(program: &mut Program) {
    program.id = 0;
    for func in &mut program.funcs {
        func.span = zero_span();
        func.node.id = 0;
        func.node.name.span = zero_span();
        for p in &mut func.node.params {
            p.id = 0;
            p.name.span = zero_span();
            p.ty.span = zero_span();
        }
        func.node.return_type.span = zero_span();
        strip_ids_and_spans_block(&mut func.node.body);
    }
}

fn assert_ok_fixture(path: &str, expected: Program) {
    let (input, _) = load_fixture(path);
    let mut actual = run_parser(&input).expect("Parser failed");
    assert_distinct_node_ids(&actual);

    let mut expected = expected;
    strip_ids_and_spans_program(&mut actual);
    strip_ids_and_spans_program(&mut expected);
    colored_assert_debug(&actual, &expected);
}

fn assert_err_count(path: &str, expected_count: usize) {
    let (input, _) = load_fixture(path);
    let result = run_parser(&input);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), expected_count);
}

#[test]
fn simple_add() {
    let expected = program(vec![func(
        "add",
        vec![param("foo", TypeKind::Int), param("bar", TypeKind::Int)],
        TypeKind::Int,
        vec![return_stmt(bin(ident("foo"), BinOpKind::Add, ident("bar")))],
    )]);
    assert_ok_fixture("tests/fixtures/parser/simple_add", expected);
}

#[test]
fn precedence() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![expr_stmt(bin(
            int_lit(1),
            BinOpKind::Add,
            bin(
                bin(int_lit(5), BinOpKind::Div, int_lit(2)),
                BinOpKind::Mult,
                int_lit(100),
            ),
        ))],
    )]);
    assert_ok_fixture("tests/fixtures/parser/precedence", expected);
}

#[test]
fn unary() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![
            assign("a", None, un(UnOpKind::Not, bool_lit(true))),
            assign("b", None, un(UnOpKind::Minus, int_lit(5))),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/unary", expected);
}

#[test]
fn function_call() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![expr_stmt(call(
            "add",
            vec![arg("a", int_lit(1)), arg("b", int_lit(2))],
        ))],
    )]);
    assert_ok_fixture("tests/fixtures/parser/function_call", expected);
}

#[test]
fn struct_literal() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![assign(
            "c",
            None,
            struct_lit(
                SpecificTypeKind::Color,
                vec![
                    field("r", int_lit(1)),
                    field("g", int_lit(2)),
                    field("b", int_lit(3)),
                ],
            ),
        )],
    )]);
    assert_ok_fixture("tests/fixtures/parser/struct_literal", expected);
}

#[test]
fn if_else() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![if_stmt(
            bool_lit(true),
            vec![assign("a", None, int_lit(1))],
            vec![else_if(bool_lit(false), vec![assign("a", None, int_lit(2))])],
            Some(vec![assign("a", None, int_lit(3))]),
        )],
    )]);
    assert_ok_fixture("tests/fixtures/parser/if_else", expected);
}

#[test]
fn for_loop() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![for_stmt(
            Some(assign("i", None, int_lit(0))),
            Some(bin(ident("i"), BinOpKind::Lt, int_lit(10))),
            vec![compound_assign("i", CompoundOpKind::AddE, int_lit(1))],
            vec![assign("a", None, ident("i"))],
        )],
    )]);
    assert_ok_fixture("tests/fixtures/parser/for_loop", expected);
}

#[test]
fn assign_basic() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![
            assign("a", None, int_lit(5)),
            assign("b", Some(TypeKind::Int), int_lit(10)),
            assign("c", Some(TypeKind::Bool), bool_lit(false)),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/assign", expected);
}

#[test]
fn field_assign() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![
            assign("cam", None, get(EndpointKind::Camera)),
            assign_obj(
                access(ident("cam"), "lookfrom"),
                struct_lit(
                    SpecificTypeKind::Vec3,
                    vec![
                        field("x", int_lit(1)),
                        field("y", int_lit(2)),
                        field("z", int_lit(3)),
                    ],
                ),
            ),
            assign_obj(
                access(access(ident("cam"), "lookfrom"), "x"),
                int_lit(5),
            ),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/field_assign", expected);
}

#[test]
fn while_loop() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![
            assign("i", None, int_lit(0)),
            while_stmt(
                bin(ident("i"), BinOpKind::Lt, int_lit(10)),
                vec![compound_assign("i", CompoundOpKind::AddE, int_lit(1))],
            ),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/while_loop", expected);
}

#[test]
fn for_field_update() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![
            assign("cam", None, get(EndpointKind::Camera)),
            for_stmt(
                None,
                None,
                vec![compound_assign_obj(
                    access(ident("cam"), "x"),
                    CompoundOpKind::AddE,
                    int_lit(1),
                )],
                vec![assign("a", None, int_lit(1))],
            ),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/for_field_update", expected);
}

#[test]
fn field_access_expr() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![
            assign("cam", None, get(EndpointKind::Camera)),
            assign("x", None, access(ident("cam"), "lookfrom")),
            assign(
                "y",
                None,
                access(access(ident("cam"), "lookfrom"), "x"),
            ),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/field_access_expr", expected);
}

#[test]
fn struct_access() {
    let expected = program(vec![func(
        "main",
        vec![],
        TypeKind::Null,
        vec![
            expr_stmt(access(ident("foo"), "bar")),
            assign_obj(access(ident("foo"), "bar"), int_lit(1)),
            compound_assign_obj(access(ident("foo"), "bar"), CompoundOpKind::AddE, int_lit(2)),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/struct_access", expected);
}

#[test]
fn http_request() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![
            assign("cam", None, get(EndpointKind::Camera)),
            http_stmt(
                HTTPMethodKind::Patch,
                EndpointKind::Camera,
                struct_lit(
                    SpecificTypeKind::Camera,
                    vec![
                        field(
                            "lookfrom",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(1)),
                                    field("y", int_lit(2)),
                                    field("z", int_lit(3)),
                                ],
                            ),
                        ),
                        field(
                            "lookat",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(0)),
                                    field("y", int_lit(0)),
                                    field("z", int_lit(0)),
                                ],
                            ),
                        ),
                        field("vfov", int_lit(90)),
                        field(
                            "vup",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(0)),
                                    field("y", int_lit(1)),
                                    field("z", int_lit(0)),
                                ],
                            ),
                        ),
                        field("focus_dist", int_lit(1)),
                        field("defocus_angle", int_lit(0)),
                    ],
                ),
            ),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/http_request", expected);
}

#[test]
fn more_http_request() {
    let expected = program(vec![func(
        "foo",
        vec![],
        TypeKind::Null,
        vec![
            http_stmt(
                HTTPMethodKind::Post,
                EndpointKind::Hittable,
                struct_lit(
                    SpecificTypeKind::Sphere,
                    vec![
                        field(
                            "coord",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(1)),
                                    field("y", int_lit(2)),
                                    field("z", int_lit(3)),
                                ],
                            ),
                        ),
                        field("radius", float_lit(0.5)),
                        field("material", struct_lit(SpecificTypeKind::Lambertian, vec![])),
                    ],
                ),
            ),
            http_stmt(
                HTTPMethodKind::Put,
                EndpointKind::Camera,
                struct_lit(
                    SpecificTypeKind::Camera,
                    vec![
                        field(
                            "lookfrom",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(1)),
                                    field("y", int_lit(2)),
                                    field("z", int_lit(3)),
                                ],
                            ),
                        ),
                        field(
                            "lookat",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(0)),
                                    field("y", int_lit(0)),
                                    field("z", int_lit(0)),
                                ],
                            ),
                        ),
                        field("vfov", int_lit(90)),
                        field(
                            "vup",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(0)),
                                    field("y", int_lit(1)),
                                    field("z", int_lit(0)),
                                ],
                            ),
                        ),
                        field("focus_dist", int_lit(1)),
                        field("defocus_angle", int_lit(0)),
                    ],
                ),
            ),
            http_stmt(
                HTTPMethodKind::Patch,
                EndpointKind::Background,
                struct_lit(
                    SpecificTypeKind::Background,
                    vec![
                        field(
                            "top",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(1)),
                                    field("y", int_lit(1)),
                                    field("z", int_lit(1)),
                                ],
                            ),
                        ),
                        field(
                            "bottom",
                            struct_lit(
                                SpecificTypeKind::Vec3,
                                vec![
                                    field("x", int_lit(0)),
                                    field("y", int_lit(0)),
                                    field("z", int_lit(0)),
                                ],
                            ),
                        ),
                    ],
                ),
            ),
        ],
    )]);
    assert_ok_fixture("tests/fixtures/parser/more_http_request", expected);
}

#[test]
fn multi_func() {
    let expected = program(vec![
        func(
            "add",
            vec![param("a", TypeKind::Int), param("b", TypeKind::Int)],
            TypeKind::Int,
            vec![return_stmt(bin(ident("a"), BinOpKind::Add, ident("b")))],
        ),
        func(
            "double",
            vec![param("a", TypeKind::Int)],
            TypeKind::Int,
            vec![return_stmt(call(
                "add",
                vec![arg("a", ident("a")), arg("b", ident("a"))],
            ))],
        ),
        func(
            "main",
            vec![],
            TypeKind::Null,
            vec![assign(
                "x",
                None,
                call("double", vec![arg("a", int_lit(5))]),
            )],
        ),
    ]);
    assert_ok_fixture("tests/fixtures/parser/multi_func", expected);
}

#[test]
fn error_missing_semicol() {
    assert_err_count("tests/fixtures/parser/error_missing_semicol", 1);
}

#[test]
fn error_missing_brace() {
    assert_err_count("tests/fixtures/parser/error_missing_brace", 1);
}

#[test]
fn error_bad_assign() {
    assert_err_count("tests/fixtures/parser/error_bad_assign", 1);
}

#[test]
fn sync_fn() {
    assert_err_count("tests/fixtures/parser/sync_fn", 1);
}

#[test]
fn sync_stmt() {
    assert_err_count("tests/fixtures/parser/sync_stmt", 1);
}

#[test]
fn sync_multi_error() {
    assert_err_count("tests/fixtures/parser/sync_multi_error", 2);
}

#[test]
fn multi_error() {
    assert_err_count("tests/fixtures/parser/multi_error", 2);
}
