use std::collections::{HashMap, HashSet};
use std::mem;

use crate::ast::expression::{BinOp, BinOpKind, Endpoint, EndpointKind};
use crate::ast::traversal::walk_expr;
use crate::ast::SpecificTypeKind;
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
            self.sem_errors.push(SemanticError::UndeclaredVariable(name.to_string()));
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
                self.sem_errors.push(SemanticError::InvalidGetRequest(*ep));
                return;
            }
        };
        let ty = TypeKind::SpecificType(spec_ty);
        self.type_table.insert(expr.id, ty);
    }

}
