use crate::{
    ast::{expression::{BinOpKind, EndpointKind, Literal, UnOpKind}, 
    statement::HTTPMethodKind, SpecificTypeKind, TypeKind}, 
    ir::{basic_block::{BasicBlock, BlockId}, Dest, Temp}
};
use std::fmt;

pub type SSABlock = BasicBlock<SSAInstruction, SSATerminator>;

#[derive(Debug)]
pub struct SSAFunctionParam {
    pub name: String, 
    pub ty: TypeKind,
}

impl fmt::Display for SSAFunctionParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

#[derive(Debug)]
pub struct SSAFunction {
    pub name: String, 
    pub params: Vec<SSAFunctionParam>,
    pub block_id: BlockId,
    pub blocks: Vec<SSABlock>,
}

impl fmt::Display for SSAFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut params_str = String::new();
        let mut first = true; 

        for param in &self.params {
            if !first {
                params_str.push_str(", ");
            }
            params_str.push_str(&param.to_string());
            first = false; 
        }
        writeln!(f, "fn {}#{}({}) {{", self.name, self.block_id, params_str)?;
        for block in &self.blocks {
            writeln!(f, "{}", block)?;
        }
        writeln!(f, "}}")
    }
}

#[derive(Debug)]
pub struct SSAProgram {
    pub functions: Vec<SSAFunction>,
}

impl fmt::Display for SSAProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function in &self.functions {
            write!(f, "{}", function.to_string())?;
        }
        Ok(())
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
pub struct SSAFieldInit {
    pub name: String,
    pub value: SSAOperand,
}

impl fmt::Display for SSAFieldInit {
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
            Self::Goto(id) => write!(f, "goto block_{id}"), 
            Self::IfGoto { cond, true_label, false_label } => 
                write!(f, "if {cond} goto block_{true_label} else block_{false_label}"),
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
        fields: Vec<SSAFieldInit>,
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
    /// When multiple code branches 
    /// merge into a single variable i.e:
    /// x = 0; 
    /// if (x > 1) {
    ///     x = 2;
    /// } else {
    ///     x = 3;
    /// }
    /// So now that what collapsed are 
    /// t = Phi(2, 3), as in a temp, 
    /// for x after the if produces either
    /// 2 or 3, depending on the runtime of x
    Phi {
        target: Dest, 
        args: Vec<SSAOperand>,
    },
}

impl fmt::Display for SSAInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BinOp { target, left, op, right } => 
                write!(f, "{target} = {left} {op} {right}"),
            Self::UnOp { target, op, value } => 
                write!(f, "{target} = {op}{value}"),
            Self::Call { target, args, func } => {
                // At most 4 elements I think 
                let mut args_str = String::with_capacity(args.len() * 4);
                let mut first = true; 
                for opr in args {
                    if !first {
                        args_str.push_str(", ");
                    } 
                    first = false;
                    args_str.push_str(&opr.to_string());
                }

                    
                if let Some(t) = target {
                    write!(f, "{t} = {func}({args_str})")
                } else {
                    write!(f, "{func}({args_str})")
                }
            }
            Self::FieldLoad { target, obj, key } => 
                write!(f, "{target} = {obj}->{key}"),
            Self::FieldStore { obj, key, value } => 
                write!(f, "{obj}->{key} = {value}"),
            Self::Copy { target, value } => 
                write!(f, "{target} = {value}"),
            Self::Construct { target, ty, fields } => {
                let mut fields_str = String::from("{");
                let mut first = true; 

                for field in fields {
                    if !first {
                        fields_str.push_str(", ");
                    }
                    fields_str.push_str(&field.to_string());
                    first = false; 
                }
                fields_str.push_str("}");
                write!(f, "{target} = {ty} {fields_str}")
            },
            Self::HTTPGet { target, ep } =>
                write!(f, "{target} = GET {ep}"),
            Self::HTTPWrite { method, ep, value } => 
                write!(f, "{method} {ep} {value}"),
            Self::Phi { target, args } => {
                let mut args_str = String::with_capacity(args.len() * 4); 
                let mut first = true; 

                for arg in args {
                    if !first {
                        args_str.push_str(", ");
                    }
                    args_str.push_str(&arg.to_string());
                    first = false;
                }

                write!(f, "{target} = Phi({args_str})")
            },
        }
    }
}
