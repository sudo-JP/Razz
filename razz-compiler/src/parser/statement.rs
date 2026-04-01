use crate::{ast::{statement::{FnDecl, Param, Stmt}, Spanned, Type}, common::Span, lexer::tokens::TokenKind, parser::error::ParserError};

use super::parser::Parser;

impl Parser {
    /// Param ::= IDENT ":" Type ;
    /// 
    /// Params ::= [ Param { "," Param } ] ; 
    /// 
    /// FuncDecl ::= "fn" IDENT "(" Params ")" Type Block ;
    pub(in crate::parser) fn func_decl(&mut self) -> Result<Spanned<FnDecl>, ParserError> {
        let start = self.consume(&TokenKind::Fn)?.pos;
        let name = self.consume_ident()?;

        self.consume(&TokenKind::LParen)?;

        let params = self.params()?;

        self.consume(&TokenKind::RParen)?;

        let return_type = self.consume_type()?;
        let block = self.block()?;
        let body = block.node;
        let end = block.span.end;
        let func = FnDecl {
            name, 
            params, 
            return_type, 
            body
        };
        let span = Span{start, end};
        Ok(Spanned { node: func, span })
    }

    /// Params ::= [ Param { "," Param } ] ; 
    fn params(&mut self) -> Result<Vec<Param>, ParserError> {
        let mut params: Vec<Param> = vec![];
        if matches!(self.peek().kind, TokenKind::Ident(_)) {
            params.push(self.param()?);
        } else {
            return Ok(params);
        }
        let rules = [TokenKind::Comma];
        while self.match_token(&rules) {
            params.push(self.param()?);
        }

        Ok(params)
    }

    /// Param ::= IDENT ":" Type ;
    fn param(&mut self) -> Result<Param, ParserError> {
        let name = self.consume_ident()?;
        self.consume(&TokenKind::Colon)?;
        let ty = self.consume_type()?;
        Ok(Param{ name, ty })
    }


    /// Type ::= "int" 
    /// | "float" 
    /// | "bool" 
    /// | "string" 
    /// | "null" 
    /// | SpecificType ;
    /// 
    /// SpecificType ::= "Vec3" 
    /// | "Point3" 
    /// | "Color" 
    /// | "Background" 
    /// | "Camera" 
    /// | "Output" 
    /// | "Sphere" 
    /// | "Image" ;
    fn consume_type(&mut self) -> Result<Type, ParserError> {
        let ty = match self.peek().kind {
            TokenKind::Int => Type::Int, 
            TokenKind::Float => Type::Float, 
            TokenKind::NullLit => Type::Null, 
            _ => {
                Type::SpecificType(self.assert_specific_type(self.peek())?)
            }
        };
        Ok(ty)
    }

    /// Block ::= "{" { Stmt } "}" ;
    fn block(&mut self) -> Result<Spanned<Vec<Stmt>>, ParserError> {
        let mut stmts: Vec<Stmt> = vec![];
        let start = self.consume(&TokenKind::LBrace)?.pos;
        while !self.is_at_end() && !matches!(self.peek().kind, TokenKind::RBrace) {
            match self.stmt() {
                Ok(stmt) => stmts.push(stmt), 
                Err(e) => {
                    self.add_error(e);
                    self.synchronize_stmt();
                }
            }
        }
        let end = self.consume(&TokenKind::RBrace)?.pos; 

        let span = Span{start, end};
        Ok(Spanned {node: stmts, span})
    }

    ///Stmt ::= Assign 
    ///| While 
    ///| If 
    ///| For 
    ///| Return 
    ///| CompoundAssign 
    ///| HTTPRequest 
    ///| ExprStmt ;
    fn stmt(&mut self) -> Result<Stmt, ParserError> {
        match self.peek().kind {
            TokenKind::Ident(_) => self.stmt_ident(), 
            TokenKind::While => self.parse_while(),
            TokenKind::If => self.parse_if(), 
            TokenKind::For => self.parse_for(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Post 
            | TokenKind::Put 
            | TokenKind::Patch 
            => self.http_request(),
            _ => self.expr_stmt(), 
        }
    }

    fn stmt_ident(&mut self) -> Result<Stmt, ParserError> {
        // There are a couple cases here, it can be either
        // an assignment, compount assign, or an expr
        todo!()
    }

    fn assign(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

    /// While ::= "while" Expr Block  ;
    fn parse_while(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

    ///If ::= "if" Expr Block 
    ///{ "else" "if" Expr Block }
    ///[ "else" Block ] ;
    fn parse_if(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

    ///ForSet ::= Assign | CompoundAssign ;
    ///For ::= "for" [ ForSet ] ";" [ Expr ] ";" [ ForSet { ","  ForSet } ] Block ;
    fn parse_for(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

    ///Return ::= "return" Expr ";" ;
    fn parse_return(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

    ///HTTPMethod ::= "POST"
    ///| "PUT"
    ///| "PATCH" ;
    ///
    ///Endpoint ::= "/sphere"
    ///| "/camera"
    ///| "/background"
    ///| "/image"
    ///| "/output" ;
    ///
    ///HTTPRequest ::= HTTPMethod Endpoint Expr ";" ;
    fn http_request(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

    ///ExprStmt ::= Expr ";" ;
    fn expr_stmt(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

}
