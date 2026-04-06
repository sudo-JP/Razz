use razz_compiler::ast::{expression::{Arg, BinOpKind, Endpoint, Expr, Literal, StructField, UnOpKind}, statement::{CompoundOp, ElseIf, FnDecl, HTTPMethod, Param, Stmt}, Program, SpecificType, Type};
mod common;
use common::*;

#[test]
fn simple_add() {
    let (input, _) = load_fixture("tests/fixtures/parser/simple_add");
    let actual = run_parser(&input).expect("Parser failed");


    let body = s(vec![
        s(Stmt::Return(
            s(Expr::BinOp {
                left: Box::new(Expr::Ident("foo".to_string())),
                op: BinOpKind::Add,
                right: Box::new(Expr::Ident("bar".to_string())),
            })
        ))
    ]);

    let func = FnDecl {
        name: "add".to_string(),
        params: vec![
            Param { name: "foo".to_string(), ty: Type::Int },
            Param { name: "bar".to_string(), ty: Type::Int },
        ],
        return_type: Type::Int,
        body,
    };

    let expected = Program {
        funcs: vec![s(func)],
    };

    assert_eq!(actual, expected);
}

#[test]
fn precedence() {
    let (input, _) = load_fixture("tests/fixtures/parser/precedence");
    let actual = run_parser(&input).expect("Parser failed");

    let expr = s(Expr::BinOp {
        left: Box::new(Expr::Constant(Literal::Int(1))),
        op: BinOpKind::Add,
        right: Box::new(Expr::BinOp {
            left: Box::new(Expr::BinOp {
                left: Box::new(Expr::Constant(Literal::Int(5))),
                op: BinOpKind::Div,
                right: Box::new(Expr::Constant(Literal::Int(2))),
            }),
            op: BinOpKind::Mult,
            right: Box::new(Expr::Constant(Literal::Int(100))),
        }),
    });

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![s(Stmt::Expr(expr))]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn unary() {
    let (input, _) = load_fixture("tests/fixtures/parser/unary");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::Assign {
                    name: "a".to_string(),
                    type_ann: None,
                    expr: s(Expr::UnOp {
                        op: UnOpKind::Not,
                        value: Box::new(Expr::Constant(Literal::Bool(true))),
                    }),
                }),
                s(Stmt::Assign {
                    name: "b".to_string(),
                    type_ann: None,
                    expr: s(Expr::UnOp {
                        op: UnOpKind::Minus,
                        value: Box::new(Expr::Constant(Literal::Int(5))),
                    }),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn function_call() {
    let (input, _) = load_fixture("tests/fixtures/parser/function_call");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::Expr(s(Expr::FunctionCall {
                    name: "add".to_string(),
                    args: vec![
                        Arg { name: "a".to_string(), expr: Expr::Constant(Literal::Int(1)) },
                        Arg { name: "b".to_string(), expr: Expr::Constant(Literal::Int(2)) },
                    ],
                }))),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn struct_literal() {
    let (input, _) = load_fixture("tests/fixtures/parser/struct_literal");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::Assign {
                    name: "c".to_string(),
                    type_ann: None,
                    expr: s(Expr::StructLiteral {
                        ty: SpecificType::Color,
                        fields: vec![
                            StructField { key: "r".to_string(), value: Expr::Constant(Literal::Int(1)) },
                            StructField { key: "g".to_string(), value: Expr::Constant(Literal::Int(2)) },
                            StructField { key: "b".to_string(), value: Expr::Constant(Literal::Int(3)) },
                        ],
                    }),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn if_else() {
    let (input, _) = load_fixture("tests/fixtures/parser/if_else");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::If {
                    cond: s(Expr::Constant(Literal::Bool(true))),
                    body: s(vec![
                        s(Stmt::Assign { name: "a".to_string(), type_ann: None, expr: s(Expr::Constant(Literal::Int(1))) }),
                    ]),
                    else_ifs: vec![
                        s(ElseIf {
                            cond: s(Expr::Constant(Literal::Bool(false))),
                            body: s(vec![
                                s(Stmt::Assign { name: "a".to_string(), type_ann: None, expr: s(Expr::Constant(Literal::Int(2))) }),
                            ]),
                        }),
                    ],
                    else_body: Some(s(vec![
                        s(Stmt::Assign { name: "a".to_string(), type_ann: None, expr: s(Expr::Constant(Literal::Int(3))) }),
                    ])),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn for_loop() {
    let (input, _) = load_fixture("tests/fixtures/parser/for_loop");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::For {
                    decl: Some(s(Box::new(Stmt::Assign {
                        name: "i".to_string(),
                        type_ann: None,
                        expr: s(Expr::Constant(Literal::Int(0))),
                    }))),
                    cond: Some(s(Expr::BinOp {
                        left: Box::new(Expr::Ident("i".to_string())),
                        op: BinOpKind::Lt,
                        right: Box::new(Expr::Constant(Literal::Int(10))),
                    })),
                    update: vec![
                        s(Stmt::CompoundAssign {
                            name: "i".to_string(),
                            op: CompoundOp::AddE,
                            expr: s(Expr::Constant(Literal::Int(1))),
                        }),
                    ],
                    body: s(vec![
                        s(Stmt::Assign { name: "a".to_string(), type_ann: None, expr: s(Expr::Ident("i".to_string())) }),
                    ]),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn assign() {
    let (input, _) = load_fixture("tests/fixtures/parser/assign");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::Assign { name: "a".to_string(), type_ann: None, expr: s(Expr::Constant(Literal::Int(5))) }),
                s(Stmt::Assign { name: "b".to_string(), type_ann: Some(Type::Int), expr: s(Expr::Constant(Literal::Int(10))) }),
                s(Stmt::Assign { name: "c".to_string(), type_ann: Some(Type::Bool), expr: s(Expr::Constant(Literal::Bool(false))) }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn field_assign() {
    let (input, _) = load_fixture("tests/fixtures/parser/field_assign");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::Assign {
                    name: "cam".to_string(),
                    type_ann: None,
                    expr: s(Expr::HTTPRequest(Endpoint::Camera)),
                }),
                s(Stmt::AssignObj {
                    target: s(Expr::FieldAccess {
                        obj: Box::new(Expr::Ident("cam".to_string())),
                        key: "lookfrom".to_string(),
                    }),
                    expr: s(Expr::StructLiteral {
                        ty: SpecificType::Vec3,
                        fields: vec![
                            StructField { key: "x".to_string(), value: Expr::Constant(Literal::Int(1)) },
                            StructField { key: "y".to_string(), value: Expr::Constant(Literal::Int(2)) },
                            StructField { key: "z".to_string(), value: Expr::Constant(Literal::Int(3)) },
                        ],
                    }),
                }),
                s(Stmt::AssignObj {
                    target: s(Expr::FieldAccess {
                        obj: Box::new(Expr::FieldAccess {
                            obj: Box::new(Expr::Ident("cam".to_string())),
                            key: "lookfrom".to_string(),
                        }),
                        key: "x".to_string(),
                    }),
                    expr: s(Expr::Constant(Literal::Int(5))),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn while_loop() {
    let (input, _) = load_fixture("tests/fixtures/parser/while_loop");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::Assign {
                    name: "i".to_string(),
                    type_ann: None,
                    expr: s(Expr::Constant(Literal::Int(0))),
                }),
                s(Stmt::While {
                    cond: s(Expr::BinOp {
                        left: Box::new(Expr::Ident("i".to_string())),
                        op: BinOpKind::Lt,
                        right: Box::new(Expr::Constant(Literal::Int(10))),
                    }),
                    body: s(vec![
                        s(Stmt::CompoundAssign {
                            name: "i".to_string(),
                            op: CompoundOp::AddE,
                            expr: s(Expr::Constant(Literal::Int(1))),
                        }),
                    ]),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn for_field_update() {
    let (input, _) = load_fixture("tests/fixtures/parser/for_field_update");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::Assign {
                    name: "cam".to_string(),
                    type_ann: None,
                    expr: s(Expr::HTTPRequest(Endpoint::Camera)),
                }),
                s(Stmt::For {
                    decl: None,
                    cond: None,
                    update: vec![
                        s(Stmt::CompoundAssignObj {
                            target: s(Expr::FieldAccess {
                                obj: Box::new(Expr::Ident("cam".to_string())),
                                key: "x".to_string(),
                            }),
                            op: CompoundOp::AddE,
                            expr: s(Expr::Constant(Literal::Int(1))),
                        }),
                    ],
                    body: s(vec![
                        s(Stmt::Assign {
                            name: "a".to_string(),
                            type_ann: None,
                            expr: s(Expr::Constant(Literal::Int(1))),
                        }),
                    ]),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn field_access_expr() {
    let (input, _) = load_fixture("tests/fixtures/parser/field_access_expr");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::Assign {
                    name: "cam".to_string(),
                    type_ann: None,
                    expr: s(Expr::HTTPRequest(Endpoint::Camera)),
                }),
                s(Stmt::Assign {
                    name: "x".to_string(),
                    type_ann: None,
                    expr: s(Expr::FieldAccess {
                        obj: Box::new(Expr::Ident("cam".to_string())),
                        key: "lookfrom".to_string(),
                    }),
                }),
                s(Stmt::Assign {
                    name: "y".to_string(),
                    type_ann: None,
                    expr: s(Expr::FieldAccess {
                        obj: Box::new(Expr::FieldAccess {
                            obj: Box::new(Expr::Ident("cam".to_string())),
                            key: "lookfrom".to_string(),
                        }),
                        key: "x".to_string(),
                    }),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn error_missing_semicol() {
    let (input, _) = load_fixture("tests/fixtures/parser/error_missing_semicol");
    assert!(run_parser(&input).is_err());
}

#[test]
fn error_missing_brace() {
    let (input, _) = load_fixture("tests/fixtures/parser/error_missing_brace");
    assert!(run_parser(&input).is_err());
}

#[test]
fn error_bad_assign() {
    let (input, _) = load_fixture("tests/fixtures/parser/error_bad_assign");
    assert!(run_parser(&input).is_err());
}

#[test]
fn more_http_request() {
    let (input, _) = load_fixture("tests/fixtures/parser/more_http_request");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![s(FnDecl {
            name: "foo".to_string(),
            params: vec![],
            return_type: Type::Null,
            body: s(vec![
                s(Stmt::HTTPRequest {
                    method: HTTPMethod::Post,
                    endpoint: Endpoint::Hittable,
                    body: s(Expr::StructLiteral {
                        ty: SpecificType::Sphere,
                        fields: vec![
                            StructField { key: "coord".to_string(), value: Expr::StructLiteral {
                                ty: SpecificType::Vec3,
                                fields: vec![
                                    StructField { key: "x".to_string(), value: Expr::Constant(Literal::Int(1)) },
                                    StructField { key: "y".to_string(), value: Expr::Constant(Literal::Int(2)) },
                                    StructField { key: "z".to_string(), value: Expr::Constant(Literal::Int(3)) },
                                ],
                            }},
                            StructField { key: "radius".to_string(), value: Expr::Constant(Literal::Float(0.5)) },
                            StructField { key: "material".to_string(), value: Expr::StructLiteral {
                                ty: SpecificType::Lambertian,
                                fields: vec![],
                            }},
                        ],
                    }),
                }),
                s(Stmt::HTTPRequest {
                    method: HTTPMethod::Put,
                    endpoint: Endpoint::Camera,
                    body: s(Expr::StructLiteral {
                        ty: SpecificType::Camera,
                        fields: vec![
                            StructField { key: "lookfrom".to_string(), value: Expr::StructLiteral {
                                ty: SpecificType::Vec3,
                                fields: vec![
                                    StructField { key: "x".to_string(), value: Expr::Constant(Literal::Int(1)) },
                                    StructField { key: "y".to_string(), value: Expr::Constant(Literal::Int(2)) },
                                    StructField { key: "z".to_string(), value: Expr::Constant(Literal::Int(3)) },
                                ],
                            }},
                            StructField { key: "lookat".to_string(), value: Expr::StructLiteral {
                                ty: SpecificType::Vec3,
                                fields: vec![
                                    StructField { key: "x".to_string(), value: Expr::Constant(Literal::Int(0)) },
                                    StructField { key: "y".to_string(), value: Expr::Constant(Literal::Int(0)) },
                                    StructField { key: "z".to_string(), value: Expr::Constant(Literal::Int(0)) },
                                ],
                            }},
                            StructField { key: "vfov".to_string(), value: Expr::Constant(Literal::Int(90)) },
                            StructField { key: "vup".to_string(), value: Expr::StructLiteral {
                                ty: SpecificType::Vec3,
                                fields: vec![
                                    StructField { key: "x".to_string(), value: Expr::Constant(Literal::Int(0)) },
                                    StructField { key: "y".to_string(), value: Expr::Constant(Literal::Int(1)) },
                                    StructField { key: "z".to_string(), value: Expr::Constant(Literal::Int(0)) },
                                ],
                            }},
                            StructField { key: "focus_dist".to_string(), value: Expr::Constant(Literal::Int(1)) },
                            StructField { key: "defocus_angle".to_string(), value: Expr::Constant(Literal::Int(0)) },
                        ],
                    }),
                }),
                s(Stmt::HTTPRequest {
                    method: HTTPMethod::Patch,
                    endpoint: Endpoint::Background,
                    body: s(Expr::StructLiteral {
                        ty: SpecificType::Background,
                        fields: vec![
                            StructField { key: "top".to_string(), value: Expr::StructLiteral {
                                ty: SpecificType::Vec3,
                                fields: vec![
                                    StructField { key: "x".to_string(), value: Expr::Constant(Literal::Int(1)) },
                                    StructField { key: "y".to_string(), value: Expr::Constant(Literal::Int(1)) },
                                    StructField { key: "z".to_string(), value: Expr::Constant(Literal::Int(1)) },
                                ],
                            }},
                            StructField { key: "bottom".to_string(), value: Expr::StructLiteral {
                                ty: SpecificType::Vec3,
                                fields: vec![
                                    StructField { key: "x".to_string(), value: Expr::Constant(Literal::Int(0)) },
                                    StructField { key: "y".to_string(), value: Expr::Constant(Literal::Int(0)) },
                                    StructField { key: "z".to_string(), value: Expr::Constant(Literal::Int(0)) },
                                ],
                            }},
                        ],
                    }),
                }),
            ]),
        })],
    };

    assert_eq!(actual, expected);
}

#[test]
fn multi_func() {
    let (input, _) = load_fixture("tests/fixtures/parser/multi_func");
    let actual = run_parser(&input).expect("Parser failed");

    let expected = Program {
        funcs: vec![
            s(FnDecl {
                name: "add".to_string(),
                params: vec![
                    Param { name: "a".to_string(), ty: Type::Int },
                    Param { name: "b".to_string(), ty: Type::Int },
                ],
                return_type: Type::Int,
                body: s(vec![
                    s(Stmt::Return(s(Expr::BinOp {
                        left: Box::new(Expr::Ident("a".to_string())),
                        op: BinOpKind::Add,
                        right: Box::new(Expr::Ident("b".to_string())),
                    }))),
                ]),
            }),
            s(FnDecl {
                name: "double".to_string(),
                params: vec![
                    Param { name: "a".to_string(), ty: Type::Int },
                ],
                return_type: Type::Int,
                body: s(vec![
                    s(Stmt::Return(s(Expr::FunctionCall {
                        name: "add".to_string(),
                        args: vec![
                            Arg { name: "a".to_string(), expr: Expr::Ident("a".to_string()) },
                            Arg { name: "b".to_string(), expr: Expr::Ident("a".to_string()) },
                        ],
                    }))),
                ]),
            }),
            s(FnDecl {
                name: "main".to_string(),
                params: vec![],
                return_type: Type::Null,
                body: s(vec![
                    s(Stmt::Assign {
                        name: "x".to_string(),
                        type_ann: None,
                        expr: s(Expr::FunctionCall {
                            name: "double".to_string(),
                            args: vec![
                                Arg { name: "a".to_string(), expr: Expr::Constant(Literal::Int(5)) },
                            ],
                        }),
                    }),
                ]),
            }),
        ],
    };

    assert_eq!(actual, expected);
}

#[test]
fn sync_fn() {
    let (input, _) = load_fixture("tests/fixtures/parser/sync_fn");
    let result = run_parser(&input);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
}

#[test]
fn sync_stmt() {
    let (input, _) = load_fixture("tests/fixtures/parser/sync_stmt");
    let result = run_parser(&input);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
}

#[test]
fn sync_multi_error() {
    let (input, _) = load_fixture("tests/fixtures/parser/sync_multi_error");
    let result = run_parser(&input);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2);
}
