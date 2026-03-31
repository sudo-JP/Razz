use crate::{ast::{expression::{Endpoint, SpecificType}, statement::FnDecl, Program, Spanned}, common::Span, lexer::tokens::{Token, TokenKind}, parser::error::{ParserError, ParserErrorKind}};

pub struct Parser {
    tokens: Vec<Token>,
    parser_errors: Vec<ParserError>,
    current: usize
}


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, parser_errors: Vec::new() }
    }

    pub fn parse(mut self) -> Result<Program, Vec<ParserError>> {
        let mut funcs: Vec<Spanned<FnDecl>> = vec![];
        // Run until no functions def are found 
        while !self.is_at_end() {
            if let TokenKind::Fn = self.peek().kind {
                // Check for parse error 
                match self.func_decl() {
                    Ok(f) => funcs.push(f),
                    Err(e) => {
                        self.synchronize_fn();
                        self.parser_errors.push(e);
                    }
                };
            } else {
                self.parser_errors.push(self.error(self.peek()));
                self.synchronize_fn();
            }
        }
        if !self.parser_errors.is_empty() {
            Err(self.parser_errors)
        } else {
            Ok(Program { funcs })
        }
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

    /// Peek next look ahead by 1, this is a LL(2) parser
    pub (in crate::parser) fn peek_next(&self) -> &Token {
        if self.current + 1 >= self.tokens.len() {
            // Must have EOF 
            &self.tokens.last().unwrap()
        } else { &self.tokens[self.current + 1] }
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

    pub(in crate::parser) fn consume(&mut self, t: &TokenKind) -> Result<&Token, ParserError> {
        if self.check(t) { Ok(self.advance()) }
        else { 
            let token = self.peek();
            Err(self.error(token)) 
        }
    }

    pub(in crate::parser) fn consume_ident(&mut self) -> Result<String, ParserError> {
        let token = self.peek();
        if let TokenKind::Ident(s) = &token.kind {
            let string = s.to_string();
            self.advance();
            return Ok(string);
        }
        Err(self.error(token))
    }

    pub(in crate::parser) fn consume_endpoint(&mut self) -> Result<Endpoint, ParserError> {
        let token = self.peek();
        let endpoint = match &token.kind {
            TokenKind::EPCamera => Endpoint::Camera,
            TokenKind::EPSphere => Endpoint::Sphere,
            TokenKind::EPBackground => Endpoint::Background,
            TokenKind::EPImage => Endpoint::Image, 
            TokenKind::EPOutput => Endpoint::Output,
            _ => { return Err(self.error(token)) }
        };

        self.advance();
        Ok(endpoint)
    }

    pub(in crate::parser) fn error(&self, token: &Token) -> ParserError {
        let span = Span{start: token.pos, end: token.pos}; 
        let kind = ParserErrorKind::InvalidToken(token.kind.clone());
        ParserError{ span, kind }
    }

    pub(in crate::parser) fn error_at(&self, span: Span, kind: TokenKind) -> ParserError {
        let kind = ParserErrorKind::InvalidToken(kind);
        ParserError{ span, kind }
    }

    fn synchronize_fn(&mut self) {
        while !self.is_at_end() {
            if let TokenKind::Fn = self.peek().kind {
                return; 
            }
            self.advance(); 
        }
    }

    pub(in crate::parser) fn synchronize_stmt(&mut self) {
        while !self.is_at_end() {
            if let TokenKind::SemiCol = self.previous().kind {
                return;
            }

            // Valid token to fall back to 
            match self.peek().kind {
                TokenKind::Fn 
                | TokenKind::For 
                | TokenKind::If
                | TokenKind::While 
                | TokenKind::Return 
                | TokenKind::Post 
                | TokenKind::Put 
                | TokenKind::Patch => { return; },
                _ => {},
            }
            self.advance();
        }
    }

    /// SpecificType ::= "Vec3" 
    /// | "Point3" 
    /// | "Color" 
    /// | "Background" 
    /// | "Camera" 
    /// | "Output" 
    /// | "Sphere" 
    /// | "Image" ;
    pub(in crate::parser) fn match_specific_type(&self, token: &Token) -> Result<SpecificType, ParserError> {
        let ty = match token.kind {
            TokenKind::Vec3 => SpecificType::Vec3,
            TokenKind::Point3 => SpecificType::Point3,
            TokenKind::Color => SpecificType::Color,
            TokenKind::Background => SpecificType::Background,
            TokenKind::Camera => SpecificType::Camera,
            TokenKind::Output => SpecificType::Output,
            TokenKind::Sphere => SpecificType::Sphere, 
            TokenKind::Image => SpecificType::Image,
            _ => { return Err(self.error(token)); }
        }; 
        Ok(ty)
    }

}

