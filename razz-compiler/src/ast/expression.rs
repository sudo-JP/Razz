pub enum BinOpKind {
    // Arithmetic
    Add, 
    Sub, 
    Div, 
    Mult, 
    AddE, 
    SubE, 
    DivE,
    MultE,
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

pub enum UnaryOpKind {
    /// !expr
    Not, 
    /// -expr
    Minus, 
}

pub enum Literal {
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool), 
    Null,
}

pub enum HTTPMethod {
    Get, 
    Post, 
    Put, 
    Patch, 
}

pub enum Endpoint {
    Sphere, 
    Camera, 
    Background, 
    Image,
    Output,
}

pub struct Arg {
    pub name: String, 
    pub expr: Expr,
}

pub struct StructField {
    pub key: String, 
    pub value: Expr,
}

pub enum Expr {
    // Operations
    /// left + right
    BinOp {
        left: Box<Expr>,
        op: BinOpKind, 
        right: Box<Expr>
    },
    /// -expr, !expr 
    UnaryOp {
        value: Box<Expr>, 
        op: UnaryOpKind,
    },
    /// Function call func(name: expr) 
    FunctionCall {
        name: String, 
        args: Vec<Arg>,
    },
    /// Access JSON, background->color
    FieldAccess {
        obj: Box<Expr>,
        key: String,
    },
    /// Endpoint access
    HTTPRequest {
        method: HTTPMethod,
        endpoint: Endpoint,
        body: Option<Box<Expr>>,
    },
    /// Literals
    Constant(Literal),
    /// StructField { name: "Jason Phan", program: "Razz" }
    StructLiteral {
        name: String, 
        fields: Vec<StructField>,
    },
    /// Identifer name
    Identifier(String),
}
