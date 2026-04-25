use crate::{ast::{expression::{Expr}, 
    statement::{Block, CompoundOp, CompoundOpKind, ElseIf, FnDecl, HTTPMethod, HTTPMethodKind, Param, Stmt, StmtKind}, 
    Spanned, Type, TypeKind}, 
    common::Span, lexer::tokens::TokenKind, parser::error::ParserError};

use super::parser::Parser;

impl Parser {
    /// Param ::= IDENT ":" Type ;
    /// 
    /// Params ::= [ Param { "," Param } ] ; 
    /// 
    /// FuncDecl ::= "fn" IDENT "(" Params ")" Type Block ;
    pub(in crate::parser) fn func_decl(&mut self) -> Result<Spanned<FnDecl>, ParserError> {
        let start = self.consume(&TokenKind::Fn)?.span.start;
        let name = self.consume_ident()?;

        self.consume(&TokenKind::LParen)?;

        let params = self.params()?;

        self.consume(&TokenKind::RParen)?;

        let return_type = self.consume_type()?;
        let body = self.block()?;
        let end = body.span.end;
        let func = FnDecl {
            id: self.next_id(),
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
            TokenKind::Int => TypeKind::Int, 
            TokenKind::Float => TypeKind::Float, 
            TokenKind::NullLit => TypeKind::Null, 
            TokenKind::String => TypeKind::String, 
            TokenKind::Bool => TypeKind::Bool,
            _ => {
                TypeKind::SpecificType(self.assert_specific_type(self.peek())?.node)
            }
        };
        let span = self.advance().span;
        Ok(Type{ node: ty, span })
    }

    /// Block ::= "{" { Stmt } "}" ;
    fn block(&mut self) -> Result<Block, ParserError> {
        let mut stmts: Vec<Stmt> = vec![];
        let start = self.consume(&TokenKind::LBrace)?.span.start;
        while !self.is_at_end() && !self.check(&TokenKind::RBrace) {
            match self.stmt() {
                Ok(stmt) => stmts.push(stmt), 
                Err(e) => {
                    self.parser_errors.push(e);
                    self.synchronize_stmt();
                }
            }
        }
        let end = self.consume(&TokenKind::RBrace)?.span.end; 

        let span = Span{start, end};
        Ok(Block { id: self.next_id(), stmts, span })
    }

    /// Stmt ::= Assign 
    /// | While 
    /// | If 
    /// | For 
    /// | Return 
    /// | CompoundAssign 
    /// | HTTPRequest 
    /// | ExprStmt ;
    fn stmt(&mut self) -> Result<Stmt, ParserError> {
        match self.peek().kind {
            TokenKind::Ident(_) => self.stmt_assign(), 
            TokenKind::While => self.parse_while(),
            TokenKind::If => self.parse_if(), 
            TokenKind::For => self.parse_for(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Post 
            | TokenKind::Put 
            | TokenKind::Patch => self.http_request(),
            _ => self.expr_stmt(), 
        }
    }


    /// Assign ::= IDENT [ ":" Type ] "=" Expr ";" ;
    /// ExprStmt ::= Expr ";" ;
    /// AssignObj ::= IDENT "->" IDENT { "->" IDENT } "=" Expr ";" ;
    /// Where ExprStmt can be IDENT 
    fn stmt_assign(&mut self) -> Result<Stmt, ParserError> {
        let assign = self.parse_ident()?;
        let start = assign.span.start;
        let end = self.consume(&TokenKind::SemiCol)?.span.end;

        let span = Span {start, end};
        let kind = assign.kind;

        Ok(Stmt { id: self.next_id(), kind, span })
    }

    /// Assign ::= IDENT [ ":" Type ] "=" Expr ;
    /// ExprStmt ::= Expr ;
    /// AssignObj ::= IDENT "->" IDENT { "->" IDENT } "=" Expr ;
    /// Where ExprStmt can be IDENT 
    fn parse_ident(&mut self) -> Result<Stmt, ParserError> {

        // Target can be IDENT, or IDENT->IDENT
        let target = self.expression()?;
        let span = target.span;


        // There are a couple cases here, it can be either
        // an assignment, compound assign, or an expr
        match self.peek().kind {
            TokenKind::Assign => self.assign(target), 
            TokenKind::Colon => self.assign_with_type(target), 
            TokenKind::AddE
            | TokenKind::SubE 
            | TokenKind::MultE 
            | TokenKind::DivE => self.compound_assign(target), 
            _ => Ok(Stmt{
                id: self.next_id(),
                kind: StmtKind::Expr(target),
                span,
            })
        }
    }


    /// Assign ::= IDENT "=" Expr ";" ;
    /// when no type 
    fn assign(&mut self, target: Expr) -> Result<Stmt, ParserError> {
        self.consume(&TokenKind::Assign)?;
        let expr = self.expression()?;
        let end = expr.span.end;

        let span = Span{ start: target.span.start, end };
        let kind = StmtKind::Assign { target, type_ann: None, expr }; 
        Ok(Stmt{
            id: self.next_id(), 
            kind,
            span,
        })
    }

    /// Assign ::= IDENT ":" Type "=" Expr ";" ;
    /// With type annotation
    fn assign_with_type(&mut self, target: Expr) -> Result<Stmt, ParserError> {
        self.consume(&TokenKind::Colon)?;
        let type_ann = Some(self.consume_type()?);
        self.consume(&TokenKind::Assign)?;
        let expr = self.expression()?;

        let end = expr.span.end;
        let span = Span { start: target.span.start, end };
        let kind = StmtKind::Assign { target, type_ann, expr };
        Ok(Stmt{
            id: self.next_id(),
            kind, 
            span,
        })
    }

    /// CompoundOp ::= "+=" 
    /// | "-="
    /// | "*="
    /// | "/=" ;
    /// 
    /// CompoundAssign ::= IDENT CompoundOp Expr ";" ;
    fn compound_assign(&mut self, target: Expr) -> Result<Stmt, ParserError> {
        let compound_op_kind = match self.peek().kind {
            TokenKind::AddE => CompoundOpKind::AddE,
            TokenKind::SubE => CompoundOpKind::SubE, 
            TokenKind::MultE => CompoundOpKind::MultE, 
            TokenKind::DivE => CompoundOpKind::DivE, 
            _ => { return Err(self.error(self.peek())); }
        };
        let op_span = self.advance().span;
        let op = CompoundOp{
            node: compound_op_kind,
            span: op_span,
        };
        let expr = self.expression()?;
        let end = expr.span.end;
        
        let span = Span { start: target.span.start, end };
        let kind = StmtKind::CompoundAssign { target, op, expr };
        Ok(Stmt{
            id: self.next_id(),
            kind, 
            span
        })
    }

    /// While ::= "while" Expr Block  ;
    fn parse_while(&mut self) -> Result<Stmt, ParserError> {
        let start = self.consume(&TokenKind::While)?.span.start;
        let cond = self.expression()?; 
        let body = self.block()?;
        let end = body.span.end;
        
        let span = Span{start, end};
        let kind = StmtKind::While { cond, body };
        Ok(Stmt{
            id: self.next_id(),
            kind, 
            span,
        })
    }

    /// ElseIf ::= { "else" "if" Expr Block } ;
    /// 
    /// Else ::= [ "else" Block ] ;
    /// 
    /// If ::= "if" Expr Block ElseIF Else ; 
    fn parse_if(&mut self) -> Result<Stmt, ParserError> {
        let start = self.consume(&TokenKind::If)?.span.start;
        let cond = self.expression()?;
        let body = self.block()?;

        let else_ifs = self.parse_else_if()?;
        let else_body = self.parse_else()?;

        let end = if let Some(s) = &else_body {
            s.span.end
        } else if let Some(s) = else_ifs.last() {
            s.span.end
        } else {
            body.span.end
        };

        let span = Span {start, end}; 
        let kind = StmtKind::If { cond, body, else_ifs, else_body };
        Ok(Stmt{
            id: self.next_id(),
            kind, 
            span,
        })
    }

    /// ElseIf ::= { "else" "if" Expr Block } ;
    fn parse_else_if(&mut self) -> Result<Vec<ElseIf>, ParserError> {
        let mut else_ifs: Vec<ElseIf> = vec![];

        while self.check(&TokenKind::Else) && 
            let TokenKind::If = self.peek_next().kind {
                let start = self.consume(&TokenKind::Else)?.span.start;
                self.consume(&TokenKind::If)?;

                let cond = self.expression()?;
                let body = self.block()?; 
                let combined_span = Span{start, end: body.span.end};

                let node = ElseIf{
                    id: self.next_id(),
                    cond, body,
                    span: combined_span,
                };
                else_ifs.push(node);
        }
        Ok(else_ifs)
    }

    /// Else ::= [ "else" Block ] ;
    fn parse_else(&mut self) -> Result<Option<Block>, ParserError> {
        if !self.check(&TokenKind::Else) {
            return Ok(None);
        }
        self.consume(&TokenKind::Else)?;
        let block = self.block()?;

        Ok(Some(block))
    }

    /// ForSet ::= Assign | CompoundAssign | Expr ;
    /// For ::= "for" [ ForSet ] ";" [ Expr ] ";" [ ForSet { ","  ForSet } ] Block ;
    fn parse_for(&mut self) -> Result<Stmt, ParserError> {
        let start = self.consume(&TokenKind::For)?.span.start;
        // wrap decl in box
        let decl = self.option_for_clause(|this| {
            let stmt = this.parse_ident()?;
            Ok(Box::new(stmt))
        })?;
        let cond = self.option_for_clause(|this| Ok(this.expression()?))?;

        let mut update: Vec<Stmt> = vec![];
        if !self.check(&TokenKind::LBrace) {
            update.push(self.parse_ident()?);
            let rules = [TokenKind::Comma];
            while self.match_token(&rules) {
                update.push(self.parse_ident()?);
            }
        }

        let body = self.block()?;
        let end = body.span.end; 
        let span = Span{start, end};
        let kind = StmtKind::For { decl, cond, update, body };
        Ok(Stmt{
            id: self.next_id(),
            kind, 
            span,
        })
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
    fn parse_return(&mut self) -> Result<Stmt, ParserError> {
        let start = self.consume(&TokenKind::Return)?.span.start;
        let expr = self.expression()?;
        let end = self.consume(&TokenKind::SemiCol)?.span.end;

        let span = Span { start, end };
        let kind = StmtKind::Return(expr);
        Ok(Stmt{
            id: self.next_id(),
            kind, 
            span,
        })
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
    /// HTTPRequest ::= HTTPMethod Endpoint Expr ";" ;
    fn http_request(&mut self) -> Result<Stmt, ParserError> {
        let start = self.peek().span.start;
        let method = self.http_method()?;
        let endpoint = self.consume_endpoint()?;
        let body = self.expression()?;

        let end = self.consume(&TokenKind::SemiCol)?.span.end;
        let span = Span {start, end};
        let kind = StmtKind::HTTPRequest { method, endpoint, body };
        Ok(Stmt{
            id: self.next_id(),
            kind, 
            span,
        })
    }

    fn http_method(&mut self) -> Result<HTTPMethod, ParserError> {
        let method = match self.peek().kind {
            TokenKind::Post => HTTPMethodKind::Post,
            TokenKind::Put => HTTPMethodKind::Put,
            TokenKind::Patch => HTTPMethodKind::Patch, 
            _ => { return Err(self.error(self.peek())); }
        }; 
        let span = self.advance().span;
        Ok(HTTPMethod{
            node: method, 
            span,
        })
    }

    /// Does not consume ";"
    fn dangl_expr(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        let end = expr.span.end;
        let start = expr.span.start; 

        let kind = StmtKind::Expr(expr);
        let span = Span { start, end };
        Ok(Stmt{
            id: self.next_id(), 
            kind, 
            span,
        })
    }

    /// ExprStmt ::= Expr ";" ;
    fn expr_stmt(&mut self) -> Result<Stmt, ParserError> {
        let mut stmt = self.dangl_expr()?;
        let start = stmt.span.start; 
        let end = self.consume(&TokenKind::SemiCol)?.span.end;
        stmt.span = Span { start, end };

        Ok(stmt)
    }

}
