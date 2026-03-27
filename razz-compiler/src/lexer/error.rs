use crate::common::Span;
use std::fmt;

pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

// For debugging and test
impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {:?} Line: {} Col: {}", self.kind, self.span.line, self.span.col)
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
