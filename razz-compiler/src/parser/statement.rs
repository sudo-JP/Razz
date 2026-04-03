use crate::{ast::{statement::{CompoundOp, FnDecl, Param, Stmt}, Spanned, Type}, 
    common::Span, lexer::tokens::TokenKind, parser::error::ParserError};

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
        Ok(Param { name, ty })
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
        self.advance();
        Ok(ty)
    }

    /// Block ::= "{" { Stmt } "}" ;
    fn block(&mut self) -> Result<Spanned<Vec<Spanned<Stmt>>>, ParserError> {
        let mut stmts: Vec<Spanned<Stmt>> = vec![];
        let start = self.consume(&TokenKind::LBrace)?.pos;
        while !self.is_at_end() && self.check(&TokenKind::RBrace) {
            match self.stmt() {
                Ok(stmt) => stmts.push(stmt), 
                Err(e) => {
                    self.parser_errors.push(e);
                    self.synchronize_stmt();
                }
            }
        }
        let end = self.consume(&TokenKind::RBrace)?.pos; 

        let span = Span{start, end};
        Ok(Spanned {node: stmts, span})
    }

    /// Stmt ::= Assign 
    /// | While 
    /// | If 
    /// | For 
    /// | Return 
    /// | CompoundAssign 
    /// | HTTPRequest 
    /// | ExprStmt ;
    fn stmt(&mut self) -> Result<Spanned<Stmt>, ParserError> {
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

    /// Assign ::= IDENT [ ":" Type ] "=" Expr ";" ;
    /// ExprStmt ::= Expr ";" ;
    /// Where ExprStmt can be IDENT 
    fn stmt_ident(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        // There are a couple cases here, it can be either
        // an assignment, compount assign, or an expr
        match self.peek_next().kind {
            TokenKind::Eq => self.assign(), 
            TokenKind::Colon => self.assign_with_type(), 
            TokenKind::AddE
            | TokenKind::SubE 
            | TokenKind::MultE 
            | TokenKind::DivE => self.compount_assign(), 
            _ => self.expr_stmt(), 
        }
    }

    /// Assign ::= IDENT "=" Expr ";" ;
    /// when no type 
    fn assign(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.peek().pos; 
        let name = self.consume_ident()?;
        self.consume(&TokenKind::Eq)?;
        let expr = self.expression()?;
        let end = self.consume(&TokenKind::SemiCol)?.pos;

        let span = Span { start, end };
        let node = Stmt::Assign { name, type_ann: None, expr }; 
        Ok(Spanned { node, span })
    }

    /// Assign ::= IDENT ":" Type "=" Expr ";" ;
    /// With type annotation
    fn assign_with_type(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.peek().pos; 
        let name = self.consume_ident()?;  
        self.consume(&TokenKind::Colon)?;
        let type_ann = Some(self.consume_type()?);
        self.consume(&TokenKind::Eq)?;
        let expr = self.expression()?;
        let end = self.consume(&TokenKind::SemiCol)?.pos; 

        let span = Span { start, end };
        let node = Stmt::Assign { name, type_ann, expr };
        Ok(Spanned { node, span })
    }

    /// CompoundOp ::= "+=" 
    /// | "-="
    /// | "*="
    /// | "/=" ;
    /// 
    /// CompoundAssign ::= IDENT CompoundOp Expr ";" ;
    fn compount_assign(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.peek().pos; 
        let name = self.consume_ident()?; 
        let op = match self.peek().kind {
            TokenKind::AddE => CompoundOp::AddE,
            TokenKind::SubE => CompoundOp::SubE, 
            TokenKind::MultE => CompoundOp::MultE, 
            TokenKind::DivE => CompoundOp::DivE, 
            _ => { return Err(self.error(self.peek())); }
        };
        self.advance();
        let expr = self.expression()?;
        let end = self.consume(&TokenKind::SemiCol)?.pos;
        
        let span = Span { start, end };
        let node = Stmt::CompoundAssign { name, op, expr };
        Ok(Spanned { node, span })
    }

    /// While ::= "while" Expr Block  ;
    fn parse_while(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.consume(&TokenKind::While)?.pos;
        let cond = self.expression()?; 
        let block = self.block()?;
        let body = block.node;
        let end = block.span.end;
        
        let span = Span{start, end};
        let node = Stmt::While { cond, body };
        Ok(Spanned { node, span })
    }

    /// If ::= "if" Expr Block 
    /// { "else" "if" Expr Block }
    /// [ "else" Block ] ;
    fn parse_if(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        todo!()
    }

    /// ForSet ::= Assign | CompoundAssign | Expr ;
    /// For ::= "for" [ ForSet ] ";" [ Expr ] ";" [ ForSet { ","  ForSet } ] Block ;
    fn parse_for(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.consume(&TokenKind::For)?.pos;
        let decl = self.option_for_clause(|this| Ok(Box::new(this.stmt_ident()?)))?;
        let cond = self.option_for_clause(|this| Ok(this.expression()?))?;

        let mut update: Vec<Spanned<Stmt>> = vec![];
        if !self.check(&TokenKind::LBrace) {
            update.push(self.stmt_ident()?);
            let rules = [TokenKind::Comma];
            while self.match_token(&rules) {
                update.push(self.stmt_ident()?);
            }
        }

        let block = self.block()?;
        let body = block.node;
        let end = block.span.end; 
        let span = Span{start, end};
        let node = Stmt::For { decl, cond, update, body };
        Ok(Spanned { node, span })
    }

    /// Helper method for parsing for, its a bit confusing 
    /// Take in a closure, which is the current self, and return a result 
    /// consume ";" token, and return appropriate optional argument of the for, e.g [ ForSet ]
    fn option_for_clause<T, F>(&mut self, mut f: F) -> Result<Option<T>, ParserError>
    where F: FnMut(&mut Self) -> Result<T, ParserError> {
        if self.check(&TokenKind::SemiCol) {
            self.consume(&TokenKind::SemiCol)?; 
            Ok(None)
        } else {
            let res = Some(f(self)?);
            self.consume(&TokenKind::SemiCol)?; 
            Ok(res)
        }
    }

    /// Return ::= "return" Expr ";" ;
    fn parse_return(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.consume(&TokenKind::Return)?.pos;
        let expr = self.expression()?.node;
        let end = self.consume(&TokenKind::SemiCol)?.pos;

        let span = Span { start, end };
        let node = Stmt::Return(expr);
        Ok(Spanned { node, span })
    }

    /// HTTPMethod ::= "POST"
    /// | "PUT"
    /// | "PATCH" ;
    /// 
    /// Endpoint ::= "/sphere"
    /// | "/camera"
    /// | "/background"
    /// | "/image"
    /// | "/output" ;
    ///
    ///HTTPRequest ::= HTTPMethod Endpoint Expr ";" ;
    fn http_request(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        todo!()
    }

    ///ExprStmt ::= Expr ";" ;
    fn expr_stmt(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let expr = self.expression()?;
        let end = self.consume(&TokenKind::SemiCol)?.pos;
        let start = expr.span.start; 

        let node = Stmt::Expr(expr);
        let span = Span { start, end };
        Ok(Spanned { node, span })
    }

}
