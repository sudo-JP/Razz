use crate::{ast::{expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, statement::HTTPMethodKind, SpecificTypeKind, TypeKind}, ir::basic_block::BlockId};


pub enum SSATerminator {
    Return(SSAOperand), 
    Goto(BlockId), 
    IfGoto{
        cond: SSAOperand, 
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


#[derive(Clone)]
pub enum SSAOperand {
    Temp(Temp), 
    Const(Literal),
}

pub struct FieldInit {
    pub name: String,
    pub value: SSAOperand,
}

pub enum SSAInstruction {
    /// Binary Op
    /// <target> = <left> <op> <right>
    BinOp {
        target: Dest, 
        left: SSAOperand,
        op: BinOpKind,
        right: SSAOperand, 
    }, 
    /// Unary Op
    /// <target> = <op> <value>
    UnOp {
        target: Dest, 
        op: UnOpKind,
        value: SSAOperand, 
    },
    /// Function call 
    /// <target> = <func>(foo: 1, bar: 2)
    /// <func>(foo: 1, bar: 2)
    Call {
        target: Option<Dest>, 
        args: Vec<SSAOperand>,
        func: String,
    },
    /// Field Load
    /// <target> = <obj>-><key>
    FieldLoad {
        target: Dest, 
        obj: SSAOperand, 
        key: String, 
    },
    /// Field Store
    /// <obj>-><key> = <value>
    FieldStore {
        obj: SSAOperand, 
        key: String, 
        value: SSAOperand,
    },
    /// Copy, simple assignment 
    /// <target> = <value>
    Copy {
        target: Dest, 
        value: SSAOperand,
    }, 
    /// Construct for struct
    /// t1 = Color { r: t0, g: 5, b: t2 }
    /// <target> = <ty> { (<name>: <operand>)* }.
    Construct {
        target: Dest, 
        ty: SpecificTypeKind,
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
        value: SSAOperand,
    },
    Phi {
        target: Dest, 
        args: Vec<Temp>,
    },
}
