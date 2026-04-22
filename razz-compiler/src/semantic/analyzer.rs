use std::collections::{HashMap, HashSet};
use std::mem;

use crate::ast::expression::{Arg, UnOp, UnOpKind};
use crate::ast::Spanned;
use crate::ast::{expression::{BinOp, BinOpKind, Endpoint, EndpointKind},
    traversal::{walk_expr},
    SpecificTypeKind};

use crate::common::Span;
use crate::semantic::error::SemanticErrorKind;
use crate::semantic::rules::BINOP_MAP;
use crate::{ast::{expression::{Expr, Literal}, statement::FnDecl, 
    traversal::{walk_program, Walkable}, NodeId, Program, TypeKind}, 
    semantic::{error::SemanticError, symbols::SymbolTable}};

/// Keep life time of ast nodes so I don't clone it every time 
pub struct SemanticAnalyzer<'ast> {
    multability: HashSet<NodeId>,
    type_table: HashMap<NodeId, TypeKind>, 
    symbol_table: SymbolTable,
    fn_table: HashMap<&'ast str, &'ast FnDecl>,
    sem_errors: Vec<SemanticError>,
}

impl<'ast> SemanticAnalyzer<'ast> {
    pub fn new() -> Self {
        Self { 
            multability: HashSet::new() ,
            type_table: HashMap::new(), 
            symbol_table: SymbolTable::new(), 
            fn_table: HashMap::new(),
            sem_errors: vec![],
        }
    }

    pub fn check(&mut self, prog: &'ast Program) -> Result<HashSet<NodeId>, Vec<SemanticError>> {
        prog.funcs
            .iter()
            .for_each(|f| {
                self.fn_table.insert(&f.node.name.node, &f.node);
                self.type_table.insert(f.node.id, f.node.return_type.node);
            });
        walk_program(self, prog);

        if self.sem_errors.len() > 0 { Err(mem::take(&mut self.sem_errors)) }
        else { Ok(mem::take(&mut self.multability)) }
    }

    fn error(&mut self, kind: SemanticErrorKind, span: Span) {
        self.sem_errors.push(SemanticError{kind, span});
    }
}

/// Walk the AST tree, declaring types along each node
impl<'ast> Walkable for SemanticAnalyzer<'ast> {

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
        let Some(ty) = self.symbol_table.lookup_variable(name) else {
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
        walk_expr(self, expr);

        let Some(value) = self.type_table.get(&value.id) else {
            return;
        };

        let is_valid = match &op.node {
            UnOpKind::Not => matches!(value, TypeKind::Bool),
            UnOpKind::Minus => matches!(value, TypeKind::Int),
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
}
