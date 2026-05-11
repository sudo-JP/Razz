//! SSA Lowering from AST 
//! Implementation of this paper: https://c9x.me/compile/bib/braun13cc.pdf

use std::collections::HashMap;
use std::mem;

use crate::{ast::{expression::{BinOpKind, Endpoint, Expr, ExprKind}, 
statement::{Block, CompoundOp, CompoundOpKind, ElseIf, HTTPMethod, Stmt, StmtKind}, NodeId, TypeKind}, 
ir::{basic_block::{BasicBlock, BlockId}, ssa::{Dest, FieldInit, SSAInstruction, SSAOperand, SSATerminator, Temp}}};

type SSABlock = BasicBlock<SSAInstruction, SSATerminator>;

pub struct SSALowerer<'ast> {
    temp_counter: u32, 
    block_counter: u32, 
    blocks: Vec<SSABlock>,
    type_table: HashMap<NodeId, TypeKind>,
    curr_instrs: Vec<SSAInstruction>,

    // Braun's stuff
    current_def: HashMap<&'ast str, HashMap<BlockId, SSAOperand>>,
}

impl<'ast> SSALowerer<'ast> {
    pub fn new(type_table: HashMap<NodeId, TypeKind>) -> Self {
        Self { 
            temp_counter: 0, 
            block_counter: 0 , 
            blocks: vec![],
            type_table,
            curr_instrs: vec![],
            current_def: HashMap::new(),
        }
    }

    // Primitive functions
    fn emit(&mut self, instr: SSAInstruction) {
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

    // SSA Lowering functions
    fn write_variable(&mut self, variable: &'ast str, block_id: BlockId, value: SSAOperand) {
        self.current_def
            .entry(variable)
            .or_insert_with(HashMap::new)
            .insert(block_id, value);
    }

    fn read_variable(&mut self, variable: &'ast str, block_id: BlockId) -> SSAOperand {
        if let Some(block) = self.current_def.get(variable)
            && let Some(value) = block.get(&block_id) {
                value.clone()
        } else {
            self.read_variable_recursive(variable, block_id)
        }
    }

    fn read_variable_recursive(&mut self, variable: &'ast str, block_id: BlockId) -> SSAOperand {
        todo!()
    }

    // Expr lowering 
    fn lower_expr(&mut self, expr: &'ast Expr) -> SSAOperand {
        match &expr.kind {
            ExprKind::BinOp { lhs, op, rhs } => {
                let lhs_opr = self.lower_expr(&lhs);
                let rhs_opr = self.lower_expr(&rhs);
                let temp = self.expr_temp(expr);
                self.emit(SSAInstruction::BinOp { target: temp, left: lhs_opr, op: op.node, right: rhs_opr });
                SSAOperand::Temp(temp)
            },
            ExprKind::UnOp { op, value } => {
                let value_opr = self.lower_expr(&value); 
                let temp = self.expr_temp(expr);
                self.emit(SSAInstruction::UnOp { target: temp, op: op.node, value: value_opr });
                SSAOperand::Temp(temp)
            },
            ExprKind::FunctionCall { name, args } => {
                let temp = self.expr_temp(expr);
                let args_opr = args.iter()
                    .map(|arg| self.lower_expr(&arg.expr))
                    .collect();
                self.emit(SSAInstruction::Call { target: Some(temp), args: args_opr, func: name.node.to_string() });
                SSAOperand::Temp(temp)
            },
            ExprKind::FieldAccess { obj, key } => {
                let temp = self.expr_temp(expr);
                let obj_opr = self.lower_expr(&obj);
                self.emit(SSAInstruction::FieldLoad { target: temp, obj: obj_opr, key: key.node.to_string() });
                SSAOperand::Temp(temp)
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
                self.emit(SSAInstruction::Construct { target: temp, ty: ty.node, fields: field_init_vec });
                SSAOperand::Temp(temp)
            },
            ExprKind::HTTPRequest(ep) => {
                let temp = self.expr_temp(expr);
                self.emit(SSAInstruction::HTTPGet { target: temp, ep: ep.node });
                SSAOperand::Temp(temp)
            },
            ExprKind::Constant(lit) => SSAOperand::Const(lit.clone()),
            ExprKind::Ident(var) => self.read_variable(&var, self.block_counter),
        }
    }

    fn lower_stmt(&mut self, stmt: &'ast Stmt) {
        match &stmt.kind {
            StmtKind::Assign { target, expr, .. } => self.lower_assign(target, expr),
            StmtKind::CompoundAssign { target, op, expr } => self.lower_compound_assign(target, op, expr),
            StmtKind::While { cond, body } => self.lower_while(cond, body),
            StmtKind::For { decl, cond, update, body } => self.lower_for(decl, cond, update, body),
            StmtKind::If { cond, body, else_ifs, else_body } => self.lower_if(cond, body, else_ifs, else_body),
            StmtKind::HTTPRequest { method, endpoint, body } => self.lower_http_req(method, endpoint, body),
            StmtKind::Return(expr) => {
                let opr = self.lower_expr(expr);
                self.seal_block(SSATerminator::Return(opr));
            }, 
            StmtKind::Expr(expr) => { self.lower_expr(expr); },
        }
    }

    fn lower_assign(&mut self, target: &'ast Expr, expr: &'ast Expr) {
        let expr_opr = self.lower_expr(expr);
        match &target.kind {
            ExprKind::Ident(ident) => 
                // TODO:
                /*self.emit(SSAInstruction::Copy { 
                target: Dest::Var(ident.to_string()), value: expr_opr 
            })*/
                todo!(),
            ExprKind::FieldAccess { obj, key } => {
                let obj_opr = self.lower_expr(&obj);
                self.emit(SSAInstruction::FieldStore { obj: obj_opr, key: key.node.to_string(), value: expr_opr });
            },
            _ => unreachable!("Handled by Parser and Semantic, this case does not exist")
        };
    }

    /// i.e: target += expr 
    /// This translates to 
    /// t0 = target 
    /// t1 = t0 + expr 
    /// target = t1 
    fn lower_compound_assign(&mut self, target: &'ast Expr, op: &CompoundOp, expr: &'ast Expr) {
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
                self.emit(SSAInstruction::BinOp { target: t1, left: t0, op: desugared_op, right: expr_opr });
                // TODO:
                // self.emit(SSAInstruction::Copy { target: Dest::Var(ident.to_string()), value: SSAOperand::Temp(t1) });
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

    // TODO
    fn seal_block(&mut self, term: SSATerminator) {
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
