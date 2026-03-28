use crate::{ast::{expression::{Arg, BinOpKind, Expr, UnOpKind}, Spanned}, 
    lexer::tokens::TokenKind, parser::error::{ParserError, ParserErrorKind}};

use super::parser::Parser;

impl Parser {
    /// GRAMMAR PARSER
    /// Each of these functions are a nonterminal
    /// expression ::= logic_or ;
    pub(in crate::parser) fn expression(&mut self) -> Result<Spanned<Expr>, ParserError> {
        self.logic_or()
    }

    /// logic_or ::= logic_and { "||" logic_and } ; 
    fn logic_or(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let mut node = self.logic_and()?.node;
        let rules = [TokenKind::Or];
        let span = self.peek().span;

        while self.match_token(&rules) {
            let op = BinOpKind::Or;
            let right = Box::new(self.logic_and()?.node);
            node = Expr::BinOp{
                left: Box::new(node), 
                op,
                right,
            };
        }
        Ok(Spanned { node , span })
    }

    /// logic_and ::= equality { "&&" equality } ; 
    fn logic_and(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let mut node = self.equality()?.node; 
        let rules = [TokenKind::And];
        let span = self.peek().span;

        while self.match_token(&rules) {
            let op = BinOpKind::And;
            let right = Box::new(self.equality()?.node);
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        Ok(Spanned { node , span })
    }

    /// equality ::= comparison { ("==" | "!=") comparison } ;
    fn equality(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let mut node = self.comparison()?.node; 
        let rules = [TokenKind::Eq];
        let span = self.peek().span;

        while self.match_token(&rules) {
            let op = BinOpKind::Eq;
            let right = Box::new(self.equality()?.node);
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        Ok(Spanned { node , span })
    }

    /// comparison ::= term { ("<" | "<=" | ">" | ">=") term } ; 
    fn comparison(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let mut node = self.term()?.node; 
        let rules = [
            TokenKind::Lt, 
            TokenKind::Le, 
            TokenKind::Gt,
            TokenKind::Ge,
        ];
        let span = self.peek().span;

        while self.match_token(&rules) {
            let op = match self.previous().kind {
                TokenKind::Lt => BinOpKind::Lt, 
                TokenKind::Le => BinOpKind::Le, 
                TokenKind::Gt => BinOpKind::Gt, 
                TokenKind::Ge => BinOpKind::Ge, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span, kind}); 
                }
            };

            let right = Box::new(self.term()?.node);
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        Ok(Spanned { node , span })
    }

    /// term ::= factor { ("+" | "-") factor } ;
    fn term(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let mut node = self.factor()?.node; 
        let rules = [
            TokenKind::Add,
            TokenKind::Sub,
        ];
        let span = self.peek().span;

        while self.match_token(&rules) {
            let op = match self.previous().kind {
                TokenKind::Add => BinOpKind::Add, 
                TokenKind::Sub => BinOpKind::Sub, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span, kind}); 
                }
            };

            let right = Box::new(self.factor()?.node);
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        Ok(Spanned { node , span })
    }

    /// factor ::= unary { ("*" | "/") unary } ; 
    fn factor(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let mut node = self.unary()?.node; 
        let rules = [
            TokenKind::Mult,
            TokenKind::Div,
        ];
        let span = self.peek().span;

        while self.match_token(&rules) {
            let op = match self.previous().kind {
                TokenKind::Mult => BinOpKind::Mult, 
                TokenKind::Div => BinOpKind::Div, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span, kind}); 
                }
            };

            let right = Box::new(self.unary()?.node);
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        Ok(Spanned { node , span })
    } 

    /// unary ::= ("!" | "-") unary
    /// | field_access ;
    fn unary(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let rules = [
            TokenKind::Sub, 
            TokenKind::Not,
        ];

        if self.match_token(&rules) {
            let span = self.previous().span;
            let op = match self.previous().kind {
                TokenKind::Sub => UnOpKind::Minus, 
                TokenKind::Not => UnOpKind::Not, 
                _ => {
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span, kind}); 
                }
            };
            let value = Box::new(self.unary()?.node); 
            let node = Expr::UnOp { op, value };

            return Ok(Spanned { node, span });
        }

        self.field_access()
    }

    /// field_access ::= function_call { "->" IDENT } ; 
    fn field_access(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let mut node = self.functional_call()?.node; 
        let rules = [TokenKind::Arrow];
        let span = self.peek().span;

        while self.match_token(&rules) {
            let ident = self.consume_ident()?;
            node = Expr::FieldAccess { 
                obj: Box::new(node), 
                key: ident,
            };
        }
        Ok(Spanned { node , span })
    }

    /// function_call ::= IDENT "(" [ Arg { "," Arg } ] ")" 
    /// | primary ;
    fn functional_call(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let span = self.peek().span;
        if let TokenKind::Ident(ident) = &self.peek().kind
            && self.peek_next().kind == TokenKind::LParen
        {
            // Move pass IDENT (
            let name = ident.to_string();
            self.advance();
            self.advance();
            // We are in function
            let args = self.args()?;
            let token = self.peek();
            if !matches!(token.kind, TokenKind::RParen) {
                return Err(self.error(token));
            }
            self.advance();
            let node = Expr::FunctionCall { name, args };
            return Ok(Spanned{node, span});
        }
        self.primary() 
    }

    /// Args ::= [ Arg { "," Arg } ] ;
    fn args(&mut self) -> Result<Vec<Arg>, ParserError> {
        let mut args: Vec<Arg> = vec![];
        // First one have to manually check 
        let curr = self.peek();
        // [ Arg .. ]
        if matches!(&curr.kind, TokenKind::Ident(_)) {
            args.push(self.arg()?);
        } else {
            return Ok(args);
        }

        // { "," Arg }
        let rules = [TokenKind::Comma];
        while self.match_token(&rules) {
            args.push(self.arg()?);
        }

        Ok(args)
    }

    /// Arg ::= IDENT ":" Expr ; 
    fn arg(&mut self) -> Result<Arg, ParserError> {
        let token = self.peek();
        let TokenKind::Ident(ident) = &token.kind else {
            let span = token.span; 
            let kind = ParserErrorKind::InvalidToken(token.kind.clone());
            return Err(ParserError{ span, kind });
        };
        let name = ident.to_string();
        self.advance();
        let token = self.peek();
        if !matches!(token.kind, TokenKind::Colon) {
            return Err(self.error(token));
        }
        self.advance();
        let expr = self.expression()?;
        Ok(Arg{ name, expr: expr.node })
    }

    fn primary(&mut self) -> Result<Spanned<Expr>, ParserError> {
        todo!()
    }
}
