use std::fmt;

use crate::common::Span;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind, 
    pub span: Span,
}

// For debugging and test
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenKind::Eof => write!(f, "Eof"),
            _ => write!(f, "{:?} Start Line: {} Col: {}, End Line: {} Col: {}", self.kind, 
                self.span.start.line, self.span.start.col, self.span.end.line, self.span.end.col),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Statement
    /// fn
    Fn, 
    /// return
    Return, 
    /// for 
    For, 
    /// while
    While,
    /// if
    If, 
    /// else 
    Else, 
    // HTTP Verbs
    /// GET
    Get, 
    /// POST
    Post, 
    /// PUT
    Put, 
    /// PATCH
    Patch,  
    // Endpoints 
    /// /camera
    EPCamera, 
    /// /hittable
    EPHittable, 
    /// /background
    EPBackground, 
    /// /image
    EPImage, 
    /// /ouput
    EPOutput, 
    // Types 
    /// int
    Int, 
    /// float
    Float, 
    /// bool
    Bool, 
    /// string
    String, 
    // Very Specific Types
    /// Vec3
    Vec3, 
    /// Point3
    Point3, 
    /// Color
    Color, 
    /// Output
    Output, 
    /// Background
    Background, 
    /// Camera
    Camera, 
    /// Sphere
    Sphere, 
    /// Image 
    Image,
    /// Lambertian
    Lambertian,
    /// Metal 
    Metal, 
    /// Dielectric
    Dielectric,
    // Operators
    // Arithmetic
    /// `+`
    Add, 
    /// `-`
    Sub, 
    /// `*`
    Mult, 
    /// `/`
    Div, 
    /// `+=`
    AddE, 
    /// `-=`
    SubE, 
    /// `*=`
    MultE, 
    /// `/=`
    DivE, 
    // Boolean
    /// `==`
    Eq, 
    /// `>`
    Gt, 
    /// `>=`
    Ge, 
    /// `<`
    Lt, 
    /// `<=`
    Le, 
    /// `!=`
    Neq, 
    /// `&&`
    And, 
    /// `||`
    Or, 
    /// `!`
    Not, 
    // Misc
    /// `->`
    Arrow, 
    /// `=`
    Assign, 
    // Delimiters
    /// `{`
    LBrace, 
    /// `}`
    RBrace, 
    /// `(`
    LParen, 
    /// `)`
    RParen, 
    /// `:`
    Colon, 
    /// `,`
    Comma, 
    /// `;`
    SemiCol, 
    // Literals 
    /// e.g 1234
    IntLit(i32), 
    /// e.g 12.34
    FloatLit(f64), 
    /// e.g "hello"
    StringLit(String), 
    /// e.g true, false
    BoolLit(bool), 
    /// e.g `a`, `bc` idk user defined stuff
    Ident(String),
    /// null
    NullLit, 
    // Output Types
    /// PPM
    PPM, 
    /// Arduino
    Arduino,
    /// Terminate tokens
    Eof,
}

