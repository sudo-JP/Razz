use std::collections::HashMap;
use crate::ast::TypeKind;


pub struct SymbolTable {
    scopes: Vec<HashMap<String, TypeKind>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { scopes: vec![HashMap::new()] }
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
    pub fn declare_variable(&mut self, name: String, ty: TypeKind) {
        let last = self.scopes.last_mut()
            .expect("Symbol Table: Empty scope, can't declare variables");
            
        last.insert(name, ty);
    }

    /// Find first scope that has name
    pub fn lookup_variable(&self, name: &str) -> Option<TypeKind> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }
}
