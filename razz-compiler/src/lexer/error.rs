use std::fmt;
use crate::common::Span;

#[derive(Debug)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

// For debugging and test
impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {:?} Start Line: {} Col: {}, End Line: {} Col: {}", self.kind, 
        self.span.start.line, self.span.start.col, self.span.end.line, self.span.end.col)
    }
}

#[derive(Debug)]
pub enum LexErrorKind {
    InvalidChar(char),
    InvalidNumber,
    InvalidEndpoint(String),
    UnterminatedComment,
    UnterminatedString,
    InvalidEncoding, 
}
