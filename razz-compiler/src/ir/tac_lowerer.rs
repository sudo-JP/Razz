use std::collections::HashMap;
use std::mem;

use crate::{ast::{expression::{BinOpKind, Endpoint, Expr, ExprKind}, statement::{Block, CompoundOp, CompoundOpKind, ElseIf, HTTPMethod, Stmt, StmtKind}, NodeId, TypeKind}, ir::{basic_block::BasicBlock, tac::{Dest, FieldInit, TACInstruction, TACOperand, TACTerminator, Temp}}};

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
                self.emit(TACInstruction::BinOp { target: Dest::Temp(temp), left: lhs_opr, op: op.node, right: rhs_opr });
                TACOperand::Temp(temp)
            },
            ExprKind::UnOp { op, value } => {
                let value_opr = self.lower_expr(&value); 
                let temp = self.expr_temp(expr);
                self.emit(TACInstruction::UnOp { target: Dest::Temp(temp), op: op.node, value: value_opr });
                TACOperand::Temp(temp)
            },
            ExprKind::FunctionCall { name, args } => {
                let temp = self.expr_temp(expr);
                let args_opr = args.iter()
                    .map(|arg| self.lower_expr(&arg.expr))
                    .collect();
                self.emit(TACInstruction::Call { target: Some(Dest::Temp(temp)), args: args_opr, func: name.node.to_string() });
                TACOperand::Temp(temp)
            },
            ExprKind::FieldAccess { obj, key } => {
                let temp = self.expr_temp(expr);
                let obj_opr = self.lower_expr(&obj);
                self.emit(TACInstruction::FieldLoad { target: Dest::Temp(temp), obj: obj_opr, key: key.node.to_string() });
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
                self.emit(TACInstruction::Construct { target: Dest::Temp(temp), ty: ty.node, fields: field_init_vec });
                TACOperand::Temp(temp)
            },
            ExprKind::HTTPRequest(ep) => {
                let temp = self.expr_temp(expr);
                self.emit(TACInstruction::HTTPGet { target: Dest::Temp(temp), ep: ep.node });
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
            StmtKind::Return(expr) => {
                let opr = self.lower_expr(expr);
                self.seal_block(TACTerminator::Return(opr));
            }, 
            StmtKind::Expr(expr) => { self.lower_expr(expr); },
        }
    }

    fn lower_assign(&mut self, target: &Expr, expr: &Expr) {
        let expr_opr = self.lower_expr(expr);
        match &target.kind {
            ExprKind::Ident(ident) => self.emit(TACInstruction::Copy { 
                target: Dest::Var(ident.to_string()), value: expr_opr 
            }),
            ExprKind::FieldAccess { obj, key } => {
                let obj_opr = self.lower_expr(&obj);
                self.emit(TACInstruction::FieldStore { obj: obj_opr, key: key.node.to_string(), value: expr_opr });
            },
            _ => unreachable!("Handled by Parser and Semantic, this case does not exist")
        };
    }

    /// i.e: target += expr 
    /// This translates to 
    /// t0 = target 
    /// t1 = t0 + expr 
    /// target = t1 
    fn lower_compound_assign(&mut self, target: &Expr, op: &CompoundOp, expr: &Expr) {
        let expr_opr = self.lower_expr(expr);
        match &target.kind {
            ExprKind::Ident(ident) => {
                let t0 = self.lower_expr(target);
                let desugared_op = match &op.node {
                    CompoundOpKind::AddE => BinOpKind::Add,
                    CompoundOpKind::SubE => BinOpKind::Sub, 
                    CompoundOpKind::DivE => BinOpKind::Div, 
                    CompoundOpKind::MultE => BinOpKind::Mult,
                };
                let t1 = self.expr_temp(target);
                self.emit(TACInstruction::BinOp { target: Dest::Temp(t1), left: t0, op: desugared_op, right: expr_opr });
                self.emit(TACInstruction::Copy { target: Dest::Var(ident.to_string()), value: TACOperand::Temp(t1) });
            },
            ExprKind::FieldAccess { obj, key } => {
                todo!()
            },
            _ => unreachable!("Handled by Parser and Semantic, this case does not exist")
        };
    }

    fn lower_while(&mut self, cond: &Expr, body: &Block) {
    }

    fn lower_for(&mut self, decl: &Option<Box<Stmt>>, cond: &Option<Expr>, update: &[Stmt], body: &Block) {
    }

    fn lower_if(&mut self, cond: &Expr, body: &Block, else_ifs: &[ElseIf], else_body: &Option<Block>) {
    }

    fn lower_http_req(&mut self, method: &HTTPMethod, endpoint: &Endpoint, body: &Expr) {
    }

    fn seal_block(&mut self, term: TACTerminator) {
        let id = self.block_counter;
        self.block_counter += 1; 
        let block = BasicBlock{
            id, 
            instrs: mem::take(&mut self.curr_instrs), 
            term: term,
        };
        self.blocks.push(block);
    }

}
