use std::collections::HashMap;
use crate::ast::{NodeId, TypeKind};


// WARN: this maybe slow because we allocating string each time, use arena
// allocator in the future
pub struct SymbolTable {
    scopes: Vec<HashMap<String, (TypeKind, NodeId)>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop()
            .expect("Symbol Table: Can't pop empty scope");
    }

    /// The language allow such instance to exist 
    /// x = 1; 
    /// x = "Hello"; 
    pub fn declare_variable(&mut self, name: String, ty: TypeKind, id: NodeId) {
        let last = self.scopes.last_mut()
            .expect("Symbol Table: Empty scope, can't declare variables");
            
        last.insert(name, (ty, id));
    }

    /// Find first scope that has name
    pub fn lookup_variable(&self, name: &str) -> Option<(TypeKind, NodeId)> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }

    pub fn lookup_current_scope(&self, name: &str) -> Option<(TypeKind, NodeId)> {
        self.scopes
            .last()
            .expect("Scope not pushed properly")
            .get(name)
            .copied()
    }
}
