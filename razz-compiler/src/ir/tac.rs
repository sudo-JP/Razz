use crate::{ast::{expression::{BinOpKind, Literal}, TypeKind}, ir::basic_block::BlockId};

type TempId = u32;

pub enum TACTerminator {
    Return(Option<TACOperand>), 
    Goto(BlockId), 
    IfGoto{
        cond: TACOperand, 
        label: BlockId,
    },
}

pub struct Temp {
    pub id: TempId, 
    pub ty: TypeKind,
}

type Dest = Temp;

pub enum TACOperand {
    Temp(Temp), 
    Var(String), 
    Const(Literal),
}



pub enum TACInstruction {
    BinOp{
        target: Dest, 
        left: TACOperand,
        op: BinOpKind,
        right: TACOperand, 
    }
}
