use crate::{
    ast::{expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, 
    statement::HTTPMethodKind, SpecificTypeKind, TypeKind}, 
    ir::basic_block::BlockId
};
use std::fmt;


pub type TempId = u32;
pub type Dest = Temp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temp {
    pub id: TempId, 
    pub ty: TypeKind,
}

impl fmt::Display for Temp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "t{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SSAOperand {
    Temp(Temp), 
    Const(Literal),
}

impl fmt::Display for SSAOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Temp(t) => write!(f, "{t}"),
            Self::Const(c) => write!(f, "{c}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldInit {
    pub name: String,
    pub value: SSAOperand,
}

impl fmt::Display for FieldInit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[derive(Debug)]
pub enum SSATerminator {
    Return(SSAOperand), 
    Goto(BlockId), 
    IfGoto{
        cond: SSAOperand, 
        true_label: BlockId,
        false_label: BlockId,
    },
}

impl fmt::Display for SSATerminator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Return(opr) => write!(f, "ret {opr}"),
            Self::Goto(id) => write!(f, "goto {id}"), 
            Self::IfGoto { cond, true_label, false_label } => 
                write!(f, "if {cond} goto {true_label} else {false_label}"),
        }
    }
}

#[derive(Debug, Clone)]
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
        args: Vec<SSAOperand>,
    },
}

impl fmt::Display for SSAInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
