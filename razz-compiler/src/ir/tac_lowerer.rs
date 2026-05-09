use std::collections::HashMap;

use crate::{ast::{expression::{Expr, ExprKind, Literal}, NodeId, TypeKind}, ir::{basic_block::BasicBlock, tac::{TACInstruction, TACOperand, TACTerminator, Temp}}};

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

    fn emit(&mut self, instr: TACInstruction) {
        self.curr_instrs.push(instr);
    }

    fn new_temp(&mut self, ty: TypeKind) -> Temp {
        let id = self.temp_counter;
        self.temp_counter += 1;
        Temp { id, ty }
    }

    fn expr_temp(&mut self, expr: &Expr) -> Temp {
        let ty = self.type_table.get(&expr.id).unwrap().clone();
        self.new_temp(ty)
    }

    fn lower_expr(&mut self, expr: &Expr) -> TACOperand {
        match &expr.kind {
            ExprKind::BinOp { lhs, op, rhs } => {
                let lhs_opr = self.lower_expr(&lhs);
                let rhs_opr = self.lower_expr(&rhs);
                let temp = self.expr_temp(expr);
                self.emit(TACInstruction::BinOp { target: temp, left: lhs_opr, op: op.node, right: rhs_opr });
                TACOperand::Temp(temp)
            },
            ExprKind::UnOp { op, value } => {
                let value_opr = self.lower_expr(&value); 
                let temp = self.expr_temp(expr);
                self.emit(TACInstruction::UnOp { target: temp, op: op.node, value: value_opr });
                TACOperand::Temp(temp)
            },
            ExprKind::FunctionCall { name, args } => {
                let temp = self.expr_temp(expr);
                let args_opr = args.iter()
                    .map(|arg| self.lower_expr(&arg.expr))
                    .collect();
                self.emit(TACInstruction::Call { target: Some(temp), args: args_opr, func: name.node.to_string() });
                TACOperand::Temp(temp)
            },
            ExprKind::Constant(lit) => TACOperand::Const(lit.clone()),
            ExprKind::Ident(var) => TACOperand::Var(var.to_string()),
            _ => unreachable!()
        }
    }

}
