use std::collections::{HashMap, HashSet};

use crate::ir::{Temp, TempId, hir::{hir::HIRBlock, hir_expression::HIRExpr, hir_statement::HIRProgram, traversal::{HIRWalkable, walk_hir_block}}};

#[derive(Default)]
pub(crate) struct HIRRustPreprocess {
    seen_before: HashSet<TempId>,
    loop_carried: HashMap<TempId, bool>,
    is_loop: bool,
}

impl HIRRustPreprocess {
    pub fn get_mut_set(mut self, prog: &HIRProgram) -> HashMap<TempId, bool> {
        self.visit_program(prog); 
        self.loop_carried
    }
}

impl HIRWalkable for HIRRustPreprocess {
    fn visit_while(&mut self, _cond: &HIRExpr, block: &HIRBlock) {
        let old_state = self.is_loop;
        self.is_loop = true;  
        walk_hir_block(self, block);
        self.is_loop = old_state;
    }

    fn visit_assign(&mut self, target: &Temp, _expr: &HIRExpr) {
        if let Some(id) = self.seen_before.get(&target.id)
        && self.is_loop {
            self.loop_carried.insert(*id, false);
        } else {
            self.seen_before.insert(target.id);
        }
    }
}
