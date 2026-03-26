use crate::{ast::{expression::{BinOpKind, Expr}, Program, Spanned}, lexer::tokens::{Token, TokenKind}};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

pub enum ParseError{}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        todo!()
    }

    /// GRAMMAR PARSER
    /// Each of these functions are a nonterminal
    /// expression ::= logic_or ;
    fn expression(&mut self) -> Result<Spanned<Expr>, ParseError> {
        self.logic_or()
    }

    /// logic_or ::= logic_and { "||" logic_and } ; 
    fn logic_or(&mut self) -> Result<Spanned<Expr>, ParseError> {
        let mut expr = self.logic_and()?.node;
        let rules = [TokenKind::Or];
        let span = self.peek().span;

        while self.match_token(&rules) {
            let op = BinOpKind::Or;
            let right = Box::new(self.logic_and()?.node);
            expr = Expr::BinOp{
                left: Box::new(expr), 
                op,
                right,
            };
        }
        Ok(Spanned { node: expr, span })
    }

    fn logic_and(&mut self) -> Result<Spanned<Expr>, ParseError> {
        todo!()
    }

    /// Utilities functions
    /// Advance to next token, consume the current one 
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1 }
        self.previous()
    }

    /// Check if the current token has any given types 
    /// Consume the token if that's the case
    /// If it does not match, return false and leave the token alone
    fn match_token(&mut self, types: &[TokenKind]) -> bool {
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
    fn check(&self, t: &TokenKind) -> bool {
        if self.is_at_end() { false }
        else { self.peek().kind == *t }
    }

    /// Peek in context of the parser is the current token
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Look for token we already consumed 
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Check if the current token is Eof 
    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
}


