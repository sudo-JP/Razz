use crate::{ast::{expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, statement::HTTPMethodKind, TypeKind}, ir::basic_block::BlockId};


pub enum TACTerminator {
    Return(Option<TACOperand>), 
    Goto(BlockId), 
    IfGoto{
        cond: TACOperand, 
        label: BlockId,
    },
}

pub type TempId = u32;
pub type Dest = Temp;

#[derive(Clone, Copy)]
pub struct Temp {
    pub id: TempId, 
    pub ty: TypeKind,
}


pub enum TACOperand {
    Temp(Temp), 
    Var(String), 
    Const(Literal),
}

pub struct FieldInit {
    pub name: String,
    pub value: TACOperand,
}

/// Three address code instruction 
/// At most three address
pub enum TACInstruction {
    /// Binary Op
    /// <target> = <left> <op> <right>
    BinOp {
        target: Dest, 
        left: TACOperand,
        op: BinOpKind,
        right: TACOperand, 
    }, 
    /// Unary Op
    /// <target> = <op> <value>
    UnOp {
        target: Dest, 
        op: UnOpKind,
        value: TACOperand, 
    },
    /// Function call 
    /// <target> = <func>(foo: 1, bar: 2)
    /// <func>(foo: 1, bar: 2)
    Call {
        target: Option<Dest>, 
        args: Vec<TACOperand>,
        func: String,
    },
    /// Field Load
    /// <target> = <obj>-><key>
    FieldLoad {
        target: Dest, 
        obj: TACOperand, 
        key: String, 
    },
    /// Field Store
    /// <obj>-><key> = <value>
    FieldStore {
        obj: TACOperand, 
        key: String, 
        value: TACOperand,
    },
    /// Copy, simple assignment 
    /// <target> = <value>
    Copy {
        target: Dest, 
        value: TACOperand,
    }, 
    /// Construct for struct
    /// t1 = Color { r: t0, g: 5, b: t2 }
    /// <target> = <ty> { (<name>: <operand>)* }.
    Construct {
        target: Dest, 
        ty: TypeKind,
        fields: Vec<FieldInit>,
    },
    /// HTTP GET 
    /// <target> = GET <ep>
    HTTPGet {
        target: Dest, 
        ep: EndpointKind,
    },
    /// HTTP Write type 
    /// POST <ep> <value>
    HTTPWrite {
        method: HTTPMethodKind,
        ep: EndpointKind, 
        value: TACOperand,
    },
}
