//! SSA Lowering from AST 
//! Implementation of this paper: https://c9x.me/compile/bib/braun13cc.pdf

use std::collections::{HashMap, HashSet};
use std::mem;

use crate::{ast::{expression::{BinOpKind, Endpoint, Expr, ExprKind}, statement::{Block, CompoundOp, CompoundOpKind, ElseIf, FnDecl, HTTPMethod, Stmt, StmtKind}, NodeId, Program, TypeKind}, 
ir::{basic_block::{BasicBlock, BlockId}, ssa::{FieldInit, SSAInstruction, SSAOperand, SSATerminator, Temp}}};

type SSABlock = BasicBlock<SSAInstruction, SSATerminator>;

pub struct SSALowerer<'ast> {
    temp_counter: u32, 
    block_counter: u32, 
    blocks: Vec<SSABlock>,
    type_table: HashMap<NodeId, TypeKind>,
    curr_instrs: Vec<SSAInstruction>,
    curr_block: BlockId,

    // Braun's stuff
    current_def: HashMap<&'ast str, HashMap<BlockId, SSAOperand>>,
    sealed_block: HashSet<BlockId>,
    incomplete_phis: HashMap<BlockId, HashMap<&'ast str, SSAInstruction>>,
    preds: HashMap<BlockId, Vec<BlockId>>, // predecessor of a block
}

impl<'ast> SSALowerer<'ast> {
    pub fn new(type_table: HashMap<NodeId, TypeKind>) -> Self {
        Self { 
            temp_counter: 0, 
            block_counter: 0, 
            curr_block: 0,
            blocks: vec![],
            type_table,
            curr_instrs: vec![],
            current_def: HashMap::new(),
            sealed_block: HashSet::new(),
            incomplete_phis: HashMap::new(),
            preds: HashMap::new(),
        }
    }

    pub fn lower(mut self, prog: &'ast Program) -> Vec<SSABlock> {
        for f in &prog.funcs {
            self.lower_fn_decl(&f.node);
        }
        self.blocks
    }

    fn lower_fn_decl(&mut self, fn_decl: &'ast FnDecl) {
        // lower fn 
        
        for stmt in &fn_decl.body.stmts {
            self.lower_stmt(stmt);
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

    fn read_variable(&mut self, variable: &'ast str, variable_id: NodeId, block_id: BlockId) -> SSAOperand {
        if let Some(block) = self.current_def.get(variable)
            && let Some(value) = block.get(&block_id) {
                value.clone()
        } else {
            self.read_variable_recursive(variable, variable_id, block_id)
        }
    }

    fn read_variable_recursive(&mut self, variable: &'ast str, variable_id: NodeId, block_id: BlockId) -> SSAOperand {
        if !self.sealed_block.contains(&block_id) {
            let ty = self.type_table.get(&variable_id)
                .expect("Type must resolved within semantic");
            let val = SSAInstruction::Phi { target: self.new_temp(*ty), args: vec![] };
            self.incomplete_phis
                .entry(block_id)
                .or_insert_with(HashMap::new)
                .insert(variable, val);
            //value
            todo!()
        } else if let Some(preds) = self.preds.get(&block_id) 
        && preds.len() == 1{
            //self.read_variable(variable, variable_id, preds[0])
            todo!()
        } else {
            /*let ty = self.type_table.get(&variable_id)
                .expect("Type must resolved within semantic");
            let val = SSAInstruction::Phi { target: self..self.new_temp(ty), args: vec![] };
            self.write_variable(variable, block_id, val);
            val*/
            todo!()
        }
    }

    fn add_phi_operands(&mut self, 
        variable: &'ast str, variable_id: NodeId,   // Variables
        block_id: BlockId,                          // Blocks for preds
        target: &Temp, args: &mut Vec<SSAOperand>)  // Destructed Phi
    -> Option<SSAOperand> {
        let preds = self.preds.get(&block_id).cloned().unwrap_or_default();
        for pred in preds {
            let op = self.read_variable(variable, variable_id, pred);
            args.push(op);
        }
        self.try_remove_trivial_phi(target, args)
    }

    /// Remove trivial phi
    /// A trivial phi is a phi containing only same temp 
    /// or a phi that references itself 
    /// Arg is destructed phi
    fn try_remove_trivial_phi(&mut self, target: &Temp, args: &[SSAOperand]) -> Option<SSAOperand> {
        let mut same: Option<SSAOperand> = None;
        for op in args {
            // Self references
            // i.e t1 = Phi(t1)
            if let Some(same_temp) = &same 
                && same_temp == op {
                continue;
            } 
            // If the value reappears 
            // i.e t1 = Phi(t2, t2)
            else if let SSAOperand::Temp(t) = op 
                && t == target {
                continue;
            } 
            if same.is_some() {
                return None;
            }

            same = Some(op.clone());
        }
        // Undefined since semantic make sure variables 
        // are defined 
        let Some(same) = same else {
            unreachable!()
        };

        let users = self.replace_uses(&SSAOperand::Temp(*target), &same);
        // Find the user and replace that 
        for (block_id, instr_id) in users {
            let phi_data = {
                let block = self.blocks.iter()
                    .find(|b| b.id == block_id)
                    .expect("Valid block");
                let instr = block.instrs
                    .get(instr_id)
                    .expect("Valid instr");
                if let SSAInstruction::Phi { target, args } = instr {
                    Some((*target, args.clone()))
                } else {
                    None
                }
            };
            if let Some((target, args)) = phi_data {
                self.try_remove_trivial_phi(&target, &args);
            }
        }

        Some(same)
    }

    fn replace_uses(&mut self, old: &SSAOperand, new: &SSAOperand) -> Vec<(BlockId, usize)> {
        let mut users = vec![];

        // Lambda funcs for less boiler plate code 
        let replace_op = |op: &mut SSAOperand| {
            if op == old {
                *op = new.clone();
                true
            } else { false }   
        };

        let replace_temp = |temp: &mut Temp| {
            if let SSAOperand::Temp(t) = old 
                && t == temp {
                if let SSAOperand::Temp(new_t) = new {
                    *temp = *new_t;
                    return true; 
                }
                false
            } else { false }
        };


        for block in self.blocks.as_mut_slice() {
            for (instr_id, instr) in block.instrs.iter_mut().enumerate() {
                let should_push = match instr {
                    SSAInstruction::BinOp { target, left, right, .. } => {
                        replace_temp(target) ||
                        replace_op(left) || 
                        replace_op(right) 
                    },
                    SSAInstruction::UnOp { target, value, .. } => {
                        replace_temp(target) ||
                        replace_op(value)
                    },
                    SSAInstruction::Call { target, args, .. } => {
                        let mut has_phi = if let Some(t) = target {
                            replace_temp(t)
                        } else { false };
                        for arg in args {
                            has_phi |= replace_op(arg);
                        }
                        has_phi
                    },
                    SSAInstruction::FieldLoad { target, obj, .. } => {
                        replace_temp(target) ||
                        replace_op(obj)
                    },
                    SSAInstruction::FieldStore { obj, value, .. } => {
                        replace_op(obj) ||
                        replace_op(value)
                    },
                    SSAInstruction::Copy { target, value } => {
                        replace_temp(target) ||
                        replace_op(value)
                    },
                    SSAInstruction::Construct { target, fields, .. } => {
                        let mut has_phi = replace_temp(target);
                        for field in fields {
                            has_phi |= replace_op(&mut field.value);
                        }
                        has_phi
                    },
                    SSAInstruction::HTTPGet { target, .. } => {
                        replace_temp(target)
                    },
                    SSAInstruction::HTTPWrite { value, .. } => {
                        replace_op(value)
                    }, 
                    SSAInstruction::Phi { target, args } => {
                        let mut has_phi = replace_temp(target);
                        for arg in args {
                            has_phi |= replace_op(arg);
                        }
                        has_phi
                    },
                };
                if should_push { users.push((block.id, instr_id)); }
            }
        }
        users
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
            ExprKind::Ident(var) => self.read_variable(&var, expr.id, self.block_counter),
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
        self.curr_block = self.block_counter;
        self.block_counter += 1; 
        let block = BasicBlock{
            id: self.curr_block,
            instrs: mem::take(&mut self.curr_instrs), 
            term: term,
        };
        self.blocks.push(block);
    }

}
