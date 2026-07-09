use std::collections::{HashMap, HashSet};
use std::mem;

use crate::ast::expression::{Arg, ExprKind, StructField, UnOp, UnOpKind};
use crate::ast::statement::{Block, CompoundOp, ElseIf, HTTPMethod, HTTPMethodKind, Stmt, StmtKind};
use crate::ast::traversal::{walk_block, walk_fn_decl, walk_stmt};
use crate::ast::{Spanned, SpecificType, Type};
use crate::ast::{expression::{BinOp, BinOpKind, Endpoint, EndpointKind},
    traversal::{walk_expr},
    SpecificTypeKind};

use crate::common::Span;
use crate::semantic::error::SemanticErrorKind;
use crate::semantic::rules::{BINOP_MAP, ENDPOINT_MAP, FIELD_ACCESS_MAP};
use crate::{ast::{expression::{Expr, Literal}, statement::FnDecl, 
    traversal::{walk_program, ASTWalkable}, NodeId, Program, TypeKind}, 
    semantic::{error::SemanticError, symbols::SymbolTable}};

/// Keep life time of ast nodes so I don't clone it every time 
pub struct SemanticAnalyzer<'ast> {
    multability: HashSet<NodeId>,
    type_table: HashMap<NodeId, TypeKind>, 
    symbol_table: SymbolTable,
    fn_table: HashMap<&'ast str, &'ast FnDecl>,
    sem_errors: Vec<SemanticError>,
    curr_return_ty: Option<TypeKind>,
}

impl<'ast> SemanticAnalyzer<'ast> {
    pub fn new() -> Self {
        Self { 
            multability: HashSet::new() ,
            type_table: HashMap::new(), 
            symbol_table: SymbolTable::new(), 
            fn_table: HashMap::new(),
            sem_errors: vec![],
            curr_return_ty: None,
        }
    }

    /// Type check the program
    pub fn check(&mut self, prog: &'ast Program) -> Result<(HashSet<NodeId>, HashMap<NodeId, TypeKind>), Vec<SemanticError>> {
        prog.funcs
            .iter()
            .for_each(|f| {
                self.fn_table.insert(&f.node.name.node, &f.node);
                if let Some(_) = self.type_table.insert(f.node.id, f.node.return_type.node) {
                    self.error(SemanticErrorKind::DuplicateFn(f.node.name.node.to_string()), f.span);
                }
            });
        walk_program(self, prog);

        if self.sem_errors.len() > 0 { Err(mem::take(&mut self.sem_errors)) }
        else { Ok((mem::take(&mut self.multability), mem::take(&mut self.type_table))) }
    }

    fn error(&mut self, kind: SemanticErrorKind, span: Span) {
        self.sem_errors.push(SemanticError{kind, span});
    }

    /// Helper for validating fields 
    fn validate_fields(&mut self, fields: &[StructField], sp_ty: &SpecificTypeKind) {
        let mut valid_fields_map = FIELD_ACCESS_MAP.get(sp_ty)
            .expect( "Field map has to cover all specific types")
            .clone();
        for field in fields {
            match valid_fields_map.remove(field.key.node.as_str()) {
                None => self.error(SemanticErrorKind::InvalidKey(field.key.node.to_string()), field.key.span), 
                Some(expected_ty) => {
                    let Some(&ty) = self.type_table.get(&field.value.id) else { continue; };
                    if !ty.satisfies(&expected_ty) {
                        self.error(SemanticErrorKind::TypeMismatch { expected: expected_ty, got: ty }, field.value.span);
                    }
                }
            }
        }
    }

    /// Check if a block has a return statement
    fn block_returns(&self, block: &Block) -> bool {
        block.stmts
            .last()
            .map_or(false, |stmt| self.stmt_returns(stmt))
    }

    /// A block has a return if the last statement 
    /// is either a return (base case), or 
    /// an if statement (recursive case). 
    /// The if has to have an else which should also contains 
    /// a return to guaranteed returning
    fn stmt_returns(&self, stmt: &Stmt) -> bool {

        match &stmt.kind {
            StmtKind::Return(_) => true, 
            StmtKind::If { body, else_ifs, else_body, .. } => {
                self.block_returns(body) &&
                else_ifs
                    .iter()
                    .all(|elif| self.block_returns(&elif.body)) &&
                else_body
                    .as_ref()
                    .map_or(false, |body| self.block_returns(&body))
            }
            _ => false,
        }
    }
}

/// Walk the AST tree, declaring types along each node
impl<'ast> ASTWalkable for SemanticAnalyzer<'ast> {

    // Leaf nodes Expr
    /// Check constant in type table 
    fn visit_constant(&mut self, expr: &Expr, lit: &Literal) {
        let ty = match lit {
            Literal::Int(_) => TypeKind::Int, 
            Literal::Float(_) => TypeKind::Float, 
            Literal::String(_) => TypeKind::String, 
            Literal::Bool(_) => TypeKind::Bool,
            Literal::Null => TypeKind::Null,
        };
        self.type_table.insert(expr.id, ty);
    }

    /// Check for ident 
    fn visit_ident(&mut self, expr: &Expr, name: &str) {
        let Some((ty, _)) = self.symbol_table.lookup_variable(name) else {
            self.error(SemanticErrorKind::UndeclaredVariable(name.to_string()), expr.span);
            return;
        };
        self.type_table.insert(expr.id, ty);
    }

    /// Check for GET req
    /// Please read docs/endpoint.md for each request validation
    fn visit_get_request(&mut self, expr: &Expr, endpoint: &Endpoint) {
        let spec_ty = match &endpoint.node {
            EndpointKind::Camera => SpecificTypeKind::Camera,
            EndpointKind::Image => SpecificTypeKind::Image, 
            EndpointKind::Background => SpecificTypeKind::Background, 
            EndpointKind::Output => SpecificTypeKind::Output, 
            ep => {
                self.error(SemanticErrorKind::InvalidGetRequest(*ep), expr.span);
                return;
            }
        };
        let ty = TypeKind::SpecificType(spec_ty);
        self.type_table.insert(expr.id, ty);
    }

    // Operational 
    /// Type check bin op, allowed `+` on String 
    /// Other operations must be 
    /// int <op> int, 
    /// or float <op> float, no casting
    fn visit_bin_op(&mut self, expr: &Expr, lhs: &Expr, op: &BinOp, rhs: &Expr) {
        walk_expr(self, lhs);
        walk_expr(self, rhs);

        let Some(lhs_ty) = self.type_table.get(&lhs.id) else {
            return;
        };

        let Some(rhs_ty) = self.type_table.get(&rhs.id) else {
            return;
        };

        // Check if two sides are the same 
        if mem::discriminant(lhs_ty) != mem::discriminant(rhs_ty) {
            self.error(SemanticErrorKind::TypeMismatch{
                expected: *lhs_ty, 
                got: *rhs_ty,
            }, expr.span);
            return;
        }
        let binop_set = BINOP_MAP.get(&op.node)
            .expect("BIN OP have to go through all operations");

        if let None = binop_set.get(lhs_ty) {
            self.error(SemanticErrorKind::InvalidBinOp{ 
                ty: *lhs_ty, op: op.node,
            }, expr.span);
            return;
        }

        let ty =  match &op.node {
            BinOpKind::Eq
            | BinOpKind::Neq 
            | BinOpKind::Lt 
            | BinOpKind::Le
            | BinOpKind::Gt 
            | BinOpKind::Ge => TypeKind::Bool, 
            _ => *lhs_ty,
        };
        self.type_table.insert(expr.id, ty);
    }

    /// Allowed -int, -float, !bool 
    fn visit_un_op(&mut self, expr: &Expr, op: &UnOp, value: &Expr) {
        walk_expr(self, value);

        let Some(value) = self.type_table.get(&value.id) else {
            return;
        };

        let is_valid = match &op.node {
            UnOpKind::Not => matches!(value, TypeKind::Bool),
            UnOpKind::Minus => matches!(value, TypeKind::Int | TypeKind::Float),
        };

        if !is_valid {
            self.error(
                SemanticErrorKind::InvalidUnOp { ty: *value, op: op.node }, 
                expr.span
            );
            return;
        }
        self.type_table.insert(expr.id, *value);
    }


    /// Type check expr function call
    fn visit_func_call(&mut self, expr: &Expr, name: &Spanned<String>, args: &[Arg]) {
        // Look up function
        let Some(look_up_func) = self.fn_table.get(&name.node.as_str()) else {
            self.error(SemanticErrorKind::UndefinedFunction(name.node.to_string()), expr.span);
            return;
        };

        let return_ty = look_up_func.return_type.node;

        // Compare len
        if args.len() != look_up_func.params.len() {
            self.error(SemanticErrorKind::WrongArgCount{
                expected: look_up_func.params.len(), got: args.len()
            }, expr.span);
            return;
        }

        // Construct map to match
        let mut map: HashMap<&str, TypeKind> = HashMap::with_capacity(look_up_func.params.len());
        for param in &look_up_func.params {
            map.insert(&param.name.node.as_str(), param.ty.node);
        }

        // Check for duplicate arg
        let mut duplicate_set: HashSet<&str> = HashSet::with_capacity(args.len());
        for arg in args {
            if !duplicate_set.insert(&arg.name.node.as_str()) {
                self.error(SemanticErrorKind::DuplicateArg(arg.name.node.to_string()), arg.name.span);
                continue;
            }

            if let Some(ty) = map.get(&arg.name.node.as_str()) {
                self.visit_expr(&arg.expr);
                let Some(arg_ty) = self.type_table.get(&arg.expr.id) else {
                    return;
                };

                if arg_ty != ty {
                    self.error(SemanticErrorKind::ArgTypeMismatch{ 
                        name: arg.name.node.to_string(), 
                        expected: *ty, 
                        got: *arg_ty, 
                    }, 
                    arg.expr.span);
                }
            } else {
                self.error(SemanticErrorKind::UnknownArg(arg.name.node.to_string()), arg.name.span);
            }
        }

        self.type_table.insert(expr.id, return_ty);
    }

    /// Declaring type for field_access retrieval
    fn visit_field_access(&mut self, expr: &Expr, obj: &Expr, key: &Spanned<String>) {
        walk_expr(self, obj);
        let Some(obj_ty) = self.type_table.get(&obj.id) else {
            return;
        };

        let TypeKind::SpecificType(structure) = *obj_ty else {
            self.error(SemanticErrorKind::InvalidFieldAccess(*obj_ty), expr.span);
            return; 
        };

        let fields = FIELD_ACCESS_MAP.get(&structure)
            .expect("Map has to cover all the specific type field");

        let Some(field_ty) = fields.get(key.node.as_str()) else {
            self.error(SemanticErrorKind::InvalidFieldAccessKey(key.node.to_string()), key.span);
            return;
        };

        self.type_table.insert(expr.id, *field_ty);

    }

    fn visit_assign(&mut self, stmt: &Stmt, target: &Expr, ty: &Option<Type>, expr: &Expr) {
        walk_expr(self, expr);
        let Some(&expr_ty) = self.type_table.get(&expr.id) else {
            return; 
        };
        match &target.kind {
            ExprKind::FieldAccess { obj, .. } => {
                if let Some(type_ann) = ty {
                    self.error(SemanticErrorKind::InvalidTypeAnnotation(type_ann.node), stmt.span);
                    return;
                }
                walk_expr(self, &obj);
                let Some(t) = self.type_table.get(&obj.id) else {
                    return;
                };
                if t.satisfies(&expr_ty) {
                    self.error(SemanticErrorKind::TypeMismatch{ expected: *t, got: expr_ty }, stmt.span);
                }
            }, 
            ExprKind::Ident(t) => {
                if let Some(type_ann) = ty 
                && type_ann.node != expr_ty {
                    self.error(SemanticErrorKind::InvalidTypeAnnotation(type_ann.node), stmt.span);
                    return; 
                }
                if let Some((ty, id)) = self.symbol_table.lookup_current_scope(&t) 
                && ty == expr_ty {
                    self.multability.insert(id);
                }
                self.symbol_table.declare_variable(t.to_string(), expr_ty, expr.id);
            }, 
            _ => unreachable!("Expect to be Ident or FieldAccess"),
        }
        self.type_table.insert(target.id, expr_ty);
    }

    /// Only allowed int, float to compound assign
    fn visit_compound_assign(&mut self, stmt: &Stmt, target: &Expr, op: &CompoundOp, expr: &Expr) {
        walk_expr(self, expr);
        let Some(&expr_ty) = self.type_table.get(&expr.id) else {
            return; 
        };


        match &target.kind {
            ExprKind::FieldAccess { obj, key } => {
                let Some(obj_ty) = self.type_table.get(&obj.id) else {
                    return;
                };
                let TypeKind::SpecificType(sp_ty) = *obj_ty else {
                    return;
                };
                let field_map = FIELD_ACCESS_MAP.get(&sp_ty)
                    .expect("Field access has to cover all specific type");

                let Some(ty) = field_map.get(key.node.as_str()) else {
                    self.error(SemanticErrorKind::InvalidFieldAccessKey(key.node.to_string()), key.span);
                    return;
                };
                if !matches!(ty, 
                    TypeKind::Int
                    | TypeKind::Float) {
                    self.error(SemanticErrorKind::InvalidBinaryAssign{ on: *ty, with: op.node }, stmt.span);
                    return;
                }
                if !expr_ty.satisfies(ty) {
                    self.error(SemanticErrorKind::TypeMismatch{ expected: *ty, got: expr_ty }, stmt.span);
                    return;
                }
            },
            ExprKind::Ident(t) => {
                let Some((t_ty, id)) = self.symbol_table.lookup_current_scope(&t) else {
                    self.error(SemanticErrorKind::UndeclaredVariable(t.to_string()), target.span);
                    return;
                };
                if !matches!(t_ty, 
                    TypeKind::Int
                    | TypeKind::Float) {
                    self.error(SemanticErrorKind::InvalidBinaryAssign{ on: t_ty, with: op.node }, stmt.span);
                    return;
                }
                if !t_ty.satisfies(&expr_ty) {
                    self.error(SemanticErrorKind::TypeMismatch{ expected: t_ty, got: expr_ty }, stmt.span);
                    return;
                }
                self.multability.insert(id);
            }, 
            _ => unreachable!("Expect to be Ident or FieldAccess"),
        }
        self.type_table.insert(target.id, expr_ty);
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        self.symbol_table.push_scope();
        for param in &fn_decl.params {
            self.symbol_table.declare_variable(param.name.node.to_string(), param.ty.node, param.id);
        }
        self.curr_return_ty = Some(fn_decl.return_type.node);
        walk_fn_decl(self, fn_decl);
        self.curr_return_ty = None;
        self.symbol_table.pop_scope();
        if !self.block_returns(&fn_decl.body) {
            self.error(SemanticErrorKind::MissingReturn, fn_decl.body.span);
        }
    }

    fn visit_return(&mut self, stmt: &Stmt, expr: &Expr) {
        walk_expr(self, expr);
        let Some(expr_ty) = self.type_table.get(&expr.id) else {
            return;
        };
        let curr_ret_ty = self.curr_return_ty
            .expect("Must declare return type before visiting here");

        if *expr_ty != curr_ret_ty {
            self.error(SemanticErrorKind::TypeMismatch { expected: curr_ret_ty, got: *expr_ty }, stmt.span);
        }
    }

    /// Declare new scope
    fn visit_block(&mut self, block: &Block) {
        self.symbol_table.push_scope();
        walk_block(self, block);
        self.symbol_table.pop_scope();
    }

    /// Condition on for loop must be boolean
    fn visit_for(&mut self, _stmt: &Stmt, decl: &Option<Box<Stmt>>, cond: &Option<Expr>, update: &[Stmt], body: &Block) {
        if let Some(d) = decl {
            walk_stmt(self, &d);
        }

        if let Some(c) = cond {
            walk_expr(self, c);
            let Some(cond_ty) = self.type_table.get(&c.id) else {
                return;
            };
            if !matches!(cond_ty, TypeKind::Bool) {
                self.error(SemanticErrorKind::InvalidConditionType(*cond_ty), c.span);
            }
        }

        update.iter()
            .for_each(|upd| walk_stmt(self, upd));

        walk_block(self, body);
    }

    /// Condition on while loop must be boolean
    fn visit_while(&mut self, _stmt: &Stmt, cond: &Expr, body: &Block) {
        walk_expr(self, cond);
        let Some(cond_ty) = self.type_table.get(&cond.id) else {
            return;
        };
        if !matches!(cond_ty, TypeKind::Bool) {
            self.error(SemanticErrorKind::InvalidConditionType(*cond_ty), cond.span);
        }

        walk_block(self, body);
    }

    fn visit_if(&mut self, _stmt: &Stmt, cond: &Expr, body: &Block, else_ifs: &[ElseIf], else_body: &Option<Block>) {
        walk_expr(self, cond);
        let Some(cond_ty) = self.type_table.get(&cond.id) else {
            return;
        };
        if !matches!(cond_ty, TypeKind::Bool) {
            self.error(SemanticErrorKind::InvalidConditionType(*cond_ty), cond.span);
        }
        walk_block(self, body);

        for elif in else_ifs {
            walk_expr(self, &elif.cond);

            let Some(cond_ty) = self.type_table.get(&elif.id) else {
                return;
            };
            if !matches!(cond_ty, TypeKind::Bool) {
                self.error(SemanticErrorKind::InvalidConditionType(*cond_ty), cond.span);
            }

            walk_block(self, &elif.body);
        }

        if let Some(block) = else_body {
            walk_block(self, block);
        }
    }



    fn visit_http_request(&mut self, _stmt: &Stmt, method: &HTTPMethod, endpoint: &Endpoint, body: &Expr) {
        walk_expr(self, body);
        let Some(&expr_ty) = self.type_table.get(&body.id) else {
            return;
        };
        let ExprKind::StructLiteral{ fields, .. } = &body.kind else {
            self.error(SemanticErrorKind::ExpectedStructLiteral, body.span);
            return;
        };

        let TypeKind::SpecificType(sp_ty) = expr_ty else {
            self.error(SemanticErrorKind::InvalidRequestBody(expr_ty), body.span);
            return;
        };
        let valid_body = ENDPOINT_MAP.get(&method.node)
            .expect("Endpoint map has to cover all methods");

        let Some(valid_ty) = valid_body.get(&endpoint.node) else {
            self.error(SemanticErrorKind::InvalidEndpoint(endpoint.node), endpoint.span);
            return;
        };
        // check here
        if let None = valid_ty.get(&sp_ty) {
            self.error(SemanticErrorKind::InvalidRequestBody(expr_ty), body.span);
            return;
        }

        let err = "Field map has to cover all specific types";
        match &method.node {
            HTTPMethodKind::Post 
            | HTTPMethodKind::Put => {
                let valid_fields: Vec<_> = FIELD_ACCESS_MAP.get(&sp_ty)
                    .expect(err)
                    .into_iter()
                    .collect();

                let mut visited: HashMap<&str, (&Spanned<String>, TypeKind)> = HashMap::new();
                for f in fields {
                    let Some(&ty) = self.type_table.get(&f.value.id) else { continue; };
                    visited.insert(f.key.node.as_str(), (&f.key, ty));
                }

                for (key, val) in valid_fields {
                    match visited.remove(key) {
                        None => self.error(SemanticErrorKind::MissingField(key.to_string()), body.span), 
                        Some((s, ty)) => {
                            if !ty.satisfies(val) {
                                self.error(SemanticErrorKind::TypeMismatch{ expected: *val, got: ty }, s.span);
                            }
                        }
                    };
                }
                
                // Haha...
                for (_, (s, _)) in visited {
                    self.error(SemanticErrorKind::InvalidKey(s.node.to_string()), s.span);
                }
            }
            HTTPMethodKind::Patch => {
                self.validate_fields(fields, &sp_ty);
            }
        }
    }

    fn visit_struct_lit(&mut self, expr: &Expr, ty: &SpecificType, fields: &[StructField]) {
        for field in fields {
            walk_expr(self, &field.value);
        }
        self.validate_fields(fields, &ty.node);
        self.type_table.insert(expr.id, TypeKind::SpecificType(ty.node));
    }
}
