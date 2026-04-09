use crate::{ast::{expression::Expr, statement::{CompoundOp, ElseIf, FnDecl, HTTPMethod, Param, Stmt}, Spanned, Type}, 
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
        let body = self.block()?;
        let end = body.span.end;
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
            TokenKind::String => Type::String, 
            TokenKind::Bool => Type::Bool,
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
        while !self.is_at_end() && !self.check(&TokenKind::RBrace) {
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

    fn stmt_assign(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let assign = self.stmt_ident()?;
        let start = assign.span.start;
        let end = self.consume(&TokenKind::SemiCol)?.pos;

        let node = assign.node;
        let span = Span {start, end};

        Ok(Spanned {node, span})
    }

    /// Assign ::= IDENT [ ":" Type ] "=" Expr ";" ;
    /// ExprStmt ::= Expr ";" ;
    /// AssignObj ::= IDENT "->" IDENT { "->" IDENT } "=" Expr ";" ;
    /// Where ExprStmt can be IDENT 
    fn stmt_ident(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        // There are a couple cases here, it can be either
        // an assignment, compount assign, or an expr
        match self.peek_next().kind {
            TokenKind::Assign => self.assign(), 
            TokenKind::Colon => self.assign_with_type(), 
            TokenKind::Arrow => self.assign_object(),
            TokenKind::AddE
            | TokenKind::SubE 
            | TokenKind::MultE 
            | TokenKind::DivE => self.compount_assign(), 
            _ => self.dangl_expr(),
        }
    }

    /// AssignObj ::= IDENT "->" IDENT { "->" IDENT } "=" Expr ";" ;
    fn assign_object(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let mut node = Expr::Ident(self.consume_ident()?);
        let start = self.previous().pos; 
        let mut end = start; 
        let rules = [TokenKind::Arrow]; 

        while self.match_token(&rules) {
            let ident = self.consume_ident()?;
            end = self.previous().pos;
            node = Expr::FieldAccess { 
                obj: Box::new(node), 
                key: ident,
            };
        }
    let target = Spanned{node, span: Span{start, end}};
    match self.peek().kind {
        TokenKind::Assign => {
            self.advance();
            let expr = self.expression()?;
            let span = Span{start, end};
            Ok(Spanned { node: Stmt::AssignObj { target, expr }, span })
        },
        TokenKind::AddE | TokenKind::SubE | TokenKind::MultE | TokenKind::DivE => {
            let op = match self.peek().kind {
                TokenKind::AddE => CompoundOp::AddE,
                TokenKind::SubE => CompoundOp::SubE,
                TokenKind::MultE => CompoundOp::MultE,
                TokenKind::DivE => CompoundOp::DivE,
                _ => unreachable!(),
            };
            self.advance();
            let expr = self.expression()?;
            let span = Span{start, end};
            Ok(Spanned { node: Stmt::CompoundAssignObj { target, op, expr }, span })
        },
        _ => Err(self.error(self.peek()))
    }
    }

    /// Assign ::= IDENT "=" Expr ";" ;
    /// when no type 
    fn assign(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.peek().pos; 
        let name = self.consume_ident()?;
        self.consume(&TokenKind::Assign)?;
        let expr = self.expression()?;
        let end = expr.span.end;

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
        self.consume(&TokenKind::Assign)?;
        let expr = self.expression()?;

        let end = expr.span.end;
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
        let end = expr.span.end;
        
        let span = Span { start, end };
        let node = Stmt::CompoundAssign { name, op, expr };
        Ok(Spanned { node, span })
    }

    /// While ::= "while" Expr Block  ;
    fn parse_while(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.consume(&TokenKind::While)?.pos;
        let cond = self.expression()?; 
        let body = self.block()?;
        let end = body.span.end;
        
        let span = Span{start, end};
        let node = Stmt::While { cond, body };
        Ok(Spanned { node, span })
    }

    /// ElseIf ::= { "else" "if" Expr Block } ;
    /// 
    /// Else ::= [ "else" Block ] ;
    /// 
    /// If ::= "if" Expr Block ElseIF Else ; 
    fn parse_if(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.consume(&TokenKind::If)?.pos;
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
        let node = Stmt::If { cond, body, else_ifs, else_body };
        Ok(Spanned { node, span })
    }

    /// ElseIf ::= { "else" "if" Expr Block } ;
    fn parse_else_if(&mut self) -> Result<Vec<Spanned<ElseIf>>, ParserError> {
        let mut else_ifs: Vec<Spanned<ElseIf>> = vec![];

        while self.check(&TokenKind::Else) && 
            let TokenKind::If = self.peek_next().kind {
                let start = self.consume(&TokenKind::Else)?.pos;
                self.consume(&TokenKind::If)?;

                let cond = self.expression()?;
                let body = self.block()?; 
                let combined_span = Span{start, end: body.span.end};

                let node = ElseIf{cond, body};
                else_ifs.push(Spanned { node, span: combined_span });
        }
        Ok(else_ifs)
    }

    /// Else ::= [ "else" Block ] ;
    fn parse_else(&mut self) -> Result<Option<Spanned<Vec<Spanned<Stmt>>>>, ParserError> {
        if !self.check(&TokenKind::Else) {
            return Ok(None);
        }
        let start = self.consume(&TokenKind::Else)?.pos;
        let block = self.block()?;
        let end = block.span.end;

        let combined_span = Span{start, end};

        Ok(Some(Spanned { node: block.node, span: combined_span }))
    }

    /// ForSet ::= Assign | CompoundAssign | Expr ;
    /// For ::= "for" [ ForSet ] ";" [ Expr ] ";" [ ForSet { ","  ForSet } ] Block ;
    fn parse_for(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.consume(&TokenKind::For)?.pos;
        // wrap decl in box
        let decl = self.option_for_clause(|this| {
            let spanned = this.stmt_ident()?;
            let span = spanned.span;
            let node = Box::new(spanned.node); 
            Ok(Spanned { node, span })
        })?;
        let cond = self.option_for_clause(|this| Ok(this.expression()?))?;

        let mut update: Vec<Spanned<Stmt>> = vec![];
        if !self.check(&TokenKind::LBrace) {
            update.push(self.stmt_ident()?);
            let rules = [TokenKind::Comma];
            while self.match_token(&rules) {
                update.push(self.stmt_ident()?);
            }
        }

        let body = self.block()?;
        let end = body.span.end; 
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
        let expr = self.expression()?;
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
    /// HTTPRequest ::= HTTPMethod Endpoint Expr ";" ;
    fn http_request(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let start = self.peek().pos;
        let method = self.http_method()?;
        let endpoint = self.consume_endpoint()?;
        let body = self.expression()?;

        let end = self.consume(&TokenKind::SemiCol)?.pos;
        let span = Span {start, end};
        let node = Stmt::HTTPRequest { method, endpoint, body };
        Ok(Spanned { node, span })
    }

    fn http_method(&mut self) -> Result<HTTPMethod, ParserError> {
        let method = match self.peek().kind {
            TokenKind::Post => HTTPMethod::Post,
            TokenKind::Put => HTTPMethod::Put,
            TokenKind::Patch => HTTPMethod::Patch, 
            _ => { return Err(self.error(self.peek())); }
        }; 
        self.advance();
        Ok(method)
    }

    /// Does not consume ";"
    fn dangl_expr(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let expr = self.expression()?;
        let end = expr.span.end;
        let start = expr.span.start; 

        let node = Stmt::Expr(expr);
        let span = Span { start, end };
        Ok(Spanned { node, span })
    }

    /// ExprStmt ::= Expr ";" ;
    fn expr_stmt(&mut self) -> Result<Spanned<Stmt>, ParserError> {
        let stmt = self.dangl_expr()?;
        let node = stmt.node; 
        let start = stmt.span.start; 
        let end = self.consume(&TokenKind::SemiCol)?.pos;

        let span = Span { start, end };
        Ok(Spanned { node, span })
    }

}
