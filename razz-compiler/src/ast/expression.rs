#[derive(Debug)]
pub enum BinOpKind {
    // Arithmetic
    Add, 
    Sub, 
    Div, 
    Mult, 
    // Comparison
    Lt, 
    Le, 
    Gt, 
    Ge, 
    Eq, 
    Neq,
    // Booleans
    And, 
    Or, 
}

#[derive(Debug)]
pub enum UnOpKind {
    /// !expr
    Not, 
    /// -expr
    Minus, 
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool), 
    Null,
}


#[derive(Debug)]
pub enum Endpoint {
    Sphere, 
    Camera, 
    Background, 
    Image,
    Output,
}

/// Argument to function 
/// func((<name>: <expr>)*)
#[derive(Debug)]
pub struct Arg {
    pub name: String, 
    pub expr: Expr,
}

/// Struct field 
/// <key>: <value>
#[derive(Debug)]
pub struct StructField {
    pub key: String, 
    pub value: Expr,
}

#[derive(Debug)]
pub enum Expr {
    // Operations
    // <left> <op> <right>
    /// 1 + 2 
    BinOp {
        left: Box<Expr>,
        op: BinOpKind, 
        right: Box<Expr>
    },
    /// Unary Operation 
    /// <op><value>
    /// e.g !true
    UnOp {
        op: UnOpKind,
        value: Box<Expr>, 
    },
    /// Function Call
    /// <name>((<arg>)*)
    /// Function call func(name: expr) 
    FunctionCall {
        name: String, 
        args: Vec<Arg>,
    },
    /// Access JSON
    /// <obj>-><key>
    /// background->color
    FieldAccess {
        obj: Box<Expr>,
        key: String,
    },
    /// Endpoint access
    /// GET <endpoint> 
    /// e.g GET /camera 
    HTTPRequest(Endpoint),
    /// Literals
    Constant(Literal),
    /// <name> { (<field>)* }
    /// Struct { author: "Jason Phan", program: "Razz" }
    StructLiteral {
        name: String, 
        fields: Vec<StructField>,
    },
    /// Identifer name
    Identifier(String),
}
