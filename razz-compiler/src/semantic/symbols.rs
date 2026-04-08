use std::collections::HashMap;
use crate::ast::Type;


pub struct SymbolTable {
    scopes: Vec<HashMap<String, Type>>,
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

    pub fn declare_variable(&mut self, name: String, ty: Type) {
        let last = self.scopes.last_mut()
            .expect("Symbol Table: Empty scope, can't declare variables");
            
        last.insert(name, ty);
    }

    /// Find first scope that has name
    pub fn lookup_variable(&self, name: &str) -> Option<Type> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).copied())
    }
}
