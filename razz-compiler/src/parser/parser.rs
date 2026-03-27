use crate::{ast::Program, lexer::tokens::{Token, TokenKind}, parser::error::ParserError};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        todo!()
    }

    /// Utilities functions
    /// Advance to next token, consume the current one 
    pub(in crate::parser) fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1 }
        self.previous()
    }

    /// Check if the current token has any given types 
    /// Consume the token if that's the case
    /// If it does not match, return false and leave the token alone
    pub(in crate::parser) fn match_token(&mut self, types: &[TokenKind]) -> bool {
        for t in types {
            if self.check(t) { 
                self.advance();
                return true; 
            }
        }
        false
    }

    /// Look at the current token without consuming
    /// Look ahead for token t
    #[inline]
    pub(in crate::parser) fn check(&self, t: &TokenKind) -> bool {
        if self.is_at_end() { false }
        else { self.peek().kind == *t }
    }

    /// Peek in context of the parser is the current token
    #[inline]
    pub(in crate::parser) fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Look for token we already consumed 
    #[inline]
    pub(in crate::parser) fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Check if the current token is Eof 
    #[inline]
    pub(in crate::parser) fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
}


