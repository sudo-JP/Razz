use crate::{common::Span, lexer::tokens::TokenKind};

/// Use to propegate error 
#[derive(Debug)]
pub enum ParserErrorKind {
    InvalidToken(TokenKind),
}

#[derive(Debug)]
pub struct ParserError {
    pub span: Span,
    pub kind: ParserErrorKind, 
}
