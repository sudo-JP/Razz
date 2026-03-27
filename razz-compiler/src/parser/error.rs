use crate::{common::Span, lexer::tokens::TokenKind};

/// Use to propegate error 
pub enum ParserErrorKind {
    InvalidToken(TokenKind),
}

pub struct ParserError {
    pub span: Span,
    pub kind: ParserErrorKind, 
}
