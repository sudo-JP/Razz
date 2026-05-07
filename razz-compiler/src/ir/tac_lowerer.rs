use std::collections::HashMap;

use crate::{ast::{NodeId, TypeKind}, ir::{basic_block::BasicBlock, tac::{TACInstruction, TACTerminator}}};

pub struct TACLowerer {
    temp_counter: u32, 
    block_counter: u32, 
    blocks: Vec<BasicBlock<TACInstruction, TACTerminator>>,
    type_table: HashMap<NodeId, TypeKind>,
    curr_instrs: Vec<TACInstruction>,
}

impl TACLowerer {
    pub fn new(type_table: HashMap<NodeId, TypeKind>) -> Self {
        Self { 
            temp_counter: 0, 
            block_counter: 0 , 
            blocks: vec![],
            type_table,
            curr_instrs: vec![],
        }
    }
}
