use std::fmt;


pub struct Token {
    pub kind: TokenKind, 
    pub line: usize, 
    pub col: usize, 
}

// For debugging and test
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenKind::Eof => write!(f, "Eof"),
            _ => write!(f, "{:?} Line: {} Col: {}", self.kind, self.line, self.col),
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    // Statement
    Fn, 
    Return, 
    For, 
    If, 
    Else, 

    // HTTP Verbs
    Get, 
    Post, 
    Put, 
    Patch,  

    // Endpoints 
    EPCamera, 
    EPSphere, 
    EPBackground, 
    EPImage, 
    EPOutput, 

    // Types 
    Int, 
    Float, 
    Bool, 
    String, 

    // Very Specific Types
    Vec3, 
    Point3, 
    Color, 
    Output, 
    Background, 
    Camera, 
    Sphere, 

    // Operators
    // Arithmetic
    Add, 
    Sub, 
    Mult, 
    Div, 
    AddE, 
    SubE, 
    MultE, 
    DivE, 

    // Boolean
    Eq, 
    Gt, 
    Ge, 
    Lt, 
    Le, 
    Neq, 
    And, 
    Or, 
    Not, 

    // Misc
    Arrow, 
    Assign, 

    // Delimiters
    LBrace, 
    RBrace, 
    LParen, 
    RParen, 
    Colon, 
    Comma, 
    SemiCol, 

    // Literals 
    IntLit(i32), 
    FloatLit(f64), 
    StringLit(String), 
    BoolLit(bool), 
    Ident(String),
    NullLit, 

    // Output Types
    PPM, 
    Arduino,

    Eof,
}

