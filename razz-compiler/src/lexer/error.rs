use crate::common::Position;
use std::fmt;

#[derive(Debug)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub pos: Position,
}

// For debugging and test
impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {:?} Line: {} Col: {}", self.kind, self.pos.line, self.pos.col)
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
