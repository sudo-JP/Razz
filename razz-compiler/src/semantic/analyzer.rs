use std::collections::{HashMap, HashSet};
use std::mem;

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

}
