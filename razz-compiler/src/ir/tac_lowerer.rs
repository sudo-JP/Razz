use std::collections::HashMap;

use crate::{ast::{expression::{Endpoint, Expr, ExprKind}, statement::{Block, CompoundOp, ElseIf, HTTPMethod, Stmt, StmtKind}, NodeId, Type, TypeKind}, ir::{basic_block::BasicBlock, tac::{FieldInit, TACInstruction, TACOperand, TACTerminator, Temp}}};

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
            ExprKind::FieldAccess { obj, key } => {
                let temp = self.expr_temp(expr);
                let obj_opr = self.lower_expr(&obj);
                self.emit(TACInstruction::FieldLoad { target: temp, obj: obj_opr, key: key.node.to_string() });
                TACOperand::Temp(temp)
            },
            ExprKind::StructLiteral { ty, fields } => {
                let temp = self.expr_temp(expr);
                let field_init_vec = fields.iter()
                    .map(|field| {
                        FieldInit{
                            name: field.key.node.to_string(), 
                            value: self.lower_expr(&field.value),
                        }
                    })
                    .collect();
                self.emit(TACInstruction::Construct { target: temp, ty: ty.node, fields: field_init_vec });
                TACOperand::Temp(temp)
            },
            ExprKind::HTTPRequest(ep) => {
                let temp = self.expr_temp(expr);
                self.emit(TACInstruction::HTTPGet { target: temp, ep: ep.node });
                TACOperand::Temp(temp)
            },
            ExprKind::Constant(lit) => TACOperand::Const(lit.clone()),
            ExprKind::Ident(var) => TACOperand::Var(var.to_string()),
        }
    }

    fn lower_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Assign { target, expr, .. } => self.lower_assign(target, expr),
            StmtKind::CompoundAssign { target, op, expr } => self.lower_compound_assign(target, op, expr),
            StmtKind::While { cond, body } => self.lower_while(cond, body),
            StmtKind::For { decl, cond, update, body } => self.lower_for(decl, cond, update, body),
            StmtKind::If { cond, body, else_ifs, else_body } => self.lower_if(cond, body, else_ifs, else_body),
            StmtKind::HTTPRequest { method, endpoint, body } => self.lower_http_req(method, endpoint, body),
            StmtKind::Return(expr) => todo!(), 
            StmtKind::Expr(expr) => todo!(),
        }
    }

    fn lower_assign(&mut self, target: &Expr, expr: &Expr) {
    }

    fn lower_compound_assign(&mut self, target: &Expr, op: &CompoundOp, expr: &Expr) {
    }

    fn lower_while(&mut self, cond: &Expr, body: &Block) {
    }

    fn lower_for(&mut self, decl: &Option<Box<Stmt>>, cond: &Option<Expr>, update: &[Stmt], body: &Block) {
    }

    fn lower_if(&mut self, cond: &Expr, body: &Block, else_ifs: &[ElseIf], else_body: &Option<Block>) {
    }

    fn lower_http_req(&mut self, method: &HTTPMethod, endpoint: &Endpoint, body: &Expr) {
    }

}
