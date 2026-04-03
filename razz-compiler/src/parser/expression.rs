use crate::{ast::{expression::{Arg, BinOpKind, Expr, SpecificType, StructField, UnOpKind}, Spanned}, common::Span, lexer::tokens::TokenKind, parser::error::{ParserError, ParserErrorKind}};
use crate::ast::expression::Literal;
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
        let logic_and = self.logic_and()?;
        let mut node = logic_and.node;
        let rules = [TokenKind::Or];
        let start = logic_and.span.start;
        let mut end = start;

        while self.match_token(&rules) {
            let op = BinOpKind::Or;
            let logic_and = self.logic_and()?;
            let right = Box::new(logic_and.node);
            end = logic_and.span.end;

            node = Expr::BinOp{
                left: Box::new(node), 
                op,
                right,
            };
        }

        let span = Span{start, end};
        Ok(Spanned { node , span })
    }

    /// logic_and ::= equality { "&&" equality } ; 
    fn logic_and(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let equality = self.equality()?;
        let mut node = equality.node; 
        let start = equality.span.start;
        let mut end = start;

        let rules = [TokenKind::And];

        while self.match_token(&rules) {
            let op = BinOpKind::And;
            let equality = self.equality()?;
            end = equality.span.end;
            let right = Box::new(equality.node);
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }

        let span = Span{start, end};
        Ok(Spanned { node , span })
    }

    /// equality ::= comparison { ("==" | "!=") comparison } ;
    fn equality(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let comparison = self.comparison()?;
        let mut node = comparison.node; 
        let rules = [TokenKind::Eq];
        let start = comparison.span.start;
        let mut end = start;


        while self.match_token(&rules) {
            let op = match self.previous().kind {
                TokenKind::Eq => BinOpKind::Eq,
                TokenKind::Neq => BinOpKind::Neq,
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };
            let comparison = self.comparison()?;
            let right = Box::new(comparison.node);
            end = comparison.span.end;
            
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        let span = Span{start, end};
        Ok(Spanned { node , span })
    }

    /// comparison ::= term { ("<" | "<=" | ">" | ">=") term } ; 
    fn comparison(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let term = self.term()?;
        let mut node = term.node; 
        let rules = [
            TokenKind::Lt, 
            TokenKind::Le, 
            TokenKind::Gt,
            TokenKind::Ge,
        ];
        let start = term.span.start;
        let mut end = start;

        while self.match_token(&rules) {
            let op = match self.previous().kind {
                TokenKind::Lt => BinOpKind::Lt, 
                TokenKind::Le => BinOpKind::Le, 
                TokenKind::Gt => BinOpKind::Gt, 
                TokenKind::Ge => BinOpKind::Ge, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };
            let term = self.term()?;
            end = term.span.end;

            let right = Box::new(term.node);
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        let span = Span{start, end};
        Ok(Spanned { node , span })
    }

    /// term ::= factor { ("+" | "-") factor } ;
    fn term(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let factor = self.factor()?;
        let mut node = factor.node; 
        let rules = [
            TokenKind::Add,
            TokenKind::Sub,
        ];
        let start = factor.span.start;
        let mut end = start;

        while self.match_token(&rules) {
            let op = match self.previous().kind {
                TokenKind::Add => BinOpKind::Add, 
                TokenKind::Sub => BinOpKind::Sub, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };

            let factor = self.factor()?;
            let right = Box::new(factor.node);
            end = factor.span.end;
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        let span = Span{start, end};
        Ok(Spanned { node , span })
    }

    /// factor ::= unary { ("*" | "/") unary } ; 
    fn factor(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let unary = self.unary()?;
        let mut node = unary.node; 
        let rules = [
            TokenKind::Mult,
            TokenKind::Div,
        ];
        let start = unary.span.start;
        let mut end = start;

        while self.match_token(&rules) {
            let op = match self.previous().kind {
                TokenKind::Mult => BinOpKind::Mult, 
                TokenKind::Div => BinOpKind::Div, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };

            let unary = self.unary()?;
            let right = Box::new(unary.node);
            end = unary.span.end;
            node = Expr::BinOp { 
                left: Box::new(node), 
                op, 
                right: right,
            }
        }
        let span = Span{start, end};
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
            let start = self.previous().pos;
            let mut end = start;
            let op = match self.previous().kind {
                TokenKind::Sub => UnOpKind::Minus, 
                TokenKind::Not => UnOpKind::Not, 
                _ => {
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };
            let unary = self.unary()?;
            end = unary.span.end;
            let value = Box::new(unary.node); 
            let node = Expr::UnOp { op, value };

            let span = Span{start, end};
            return Ok(Spanned { node, span });
        }

        self.field_access()
    }

    /// field_access ::= function_call { "->" IDENT } ; 
    fn field_access(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let func_call = self.function_call()?;
        let mut node = func_call.node; 
        let rules = [TokenKind::Arrow];
        let start = func_call.span.start;
        let mut end = start;

        while self.match_token(&rules) {
            let ident = self.consume_ident()?;
            end = self.previous().pos;
            node = Expr::FieldAccess { 
                obj: Box::new(node), 
                key: ident,
            };
        }
        let span = Span{start, end};
        Ok(Spanned { node , span })
    }

    /// function_call ::= IDENT "(" [ Arg { "," Arg } ] ")" 
    /// | primary ;
    fn function_call(&mut self) -> Result<Spanned<Expr>, ParserError> {
        if let TokenKind::Ident(ident) = &self.peek().kind
            && self.peek_next().kind == TokenKind::LParen
        {
            let start = self.peek().pos;
            // Move pass IDENT (
            let name = ident.to_string();
            self.advance();
            self.advance();
            // We are in function
            let args = self.args()?;
            let token = self.peek();
            let end = token.pos;
            if self.check(&TokenKind::RParen) {
                return Err(self.error(token));
            }
            self.advance();
            let node = Expr::FunctionCall { name, args };
            let span = Span{start, end};
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

    /// Used to parse 
    /// IDENT ":" Expr ; 
    fn parse_named_expr(&mut self) -> Result<(String, Expr), ParserError> {
        let name = self.consume_ident()?;
        self.consume(&TokenKind::Colon)?;
        let expr = self.expression()?;
        Ok((name, expr.node))
    }

    /// Arg ::= IDENT ":" Expr ; 
    fn arg(&mut self) -> Result<Arg, ParserError> {
        let (name, expr) = self.parse_named_expr()?;
        Ok(Arg{ name, expr })
    }

    /// StructLiteral ::= IDENT "{" [ StructField { "," StructField } ] "}" ; 
    /// 
    /// GET_Request ::= "GET" Endpoint ; 
    /// 
    /// primary ::= IDENT 
    /// | NUMBER 
    /// | STRING 
    /// | "true"
    /// | "false"
    /// | "null"
    /// | StructLiteral 
    /// | GET_Request
    /// | "(" Expr ")" ;
    fn primary(&mut self) -> Result<Spanned<Expr>, ParserError> {
        let token = self.advance();
        let start = token.pos;
        let span = Span{start, end: start};
        let kind = token.kind.clone();

        let node = match kind {
            TokenKind::StringLit(s) => Expr::Constant(Literal::String(s)),
            TokenKind::IntLit(i) => Expr::Constant(Literal::Int(i)), 
            TokenKind::FloatLit(f) => Expr::Constant(Literal::Float(f)), 
            TokenKind::BoolLit(b) => Expr::Constant(Literal::Bool(b)), 
            TokenKind::NullLit => Expr::Constant(Literal::Null),

            TokenKind::Get => self.get_request()?,
            TokenKind::LParen => {
                let expr = self.expression()?;
                self.consume(&TokenKind::RParen)?;
                expr.node
            }, 
            TokenKind::Ident(s) => Expr::Ident(s),
            TokenKind::Vec3 
            | TokenKind::Point3
            | TokenKind::Color
            | TokenKind::Background 
            | TokenKind::Camera 
            | TokenKind::Output 
            | TokenKind::Sphere 
            | TokenKind::Image 
            => self.struct_literal()?,
            _ => { return Err(self.error_at(span, kind)); }
        };

        Ok(Spanned{ node, span })
    }

    /// GET_Request ::= "GET" Endpoint ; 
    fn get_request(&mut self) -> Result<Expr, ParserError> {
        Ok(Expr::HTTPRequest(self.consume_endpoint()?))
    }

    /// StructLiteral ::= SpecificType "{" StructFields "}" ; 
    fn struct_literal(&mut self) -> Result<Expr, ParserError> {
        let ty = self.specific_type()?;
        self.consume(&TokenKind::LBrace)?;
        let fields = self.struct_fields()?;
        self.consume(&TokenKind::RBrace)?;
        Ok(Expr::StructLiteral { ty , fields })
    }

    /// SpecificType ::= "Vec3" 
    /// | "Point3" 
    /// | "Color" 
    /// | "Background" 
    /// | "Camera" 
    /// | "Output" 
    /// | "Sphere" 
    /// | "Image" ;
    fn specific_type(&mut self) -> Result<SpecificType, ParserError> {
        self.assert_specific_type(self.previous())
    }

    /// StructFields ::= [ StructField { "," StructField } [ "," ] ] ;
    /// Same pattern as function call
    fn struct_fields(&mut self) -> Result<Vec<StructField>, ParserError> {
        let mut fields: Vec<StructField> = vec![];
        let curr = self.peek();

        // Check if fields exist 
        if matches!(&curr.kind, TokenKind::Ident(_)) {
            fields.push(self.struct_field()?);
        } else {
            return Ok(fields);
        }

        let rules = [TokenKind::Comma];
        while self.match_token(&rules) {
            if !matches!(self.peek().kind, TokenKind::Ident(_)) {
                break;
            }
            fields.push(self.struct_field()?);
        }

        Ok(fields)
    }

    /// StructField ::= IDENT ":" Expr ;
    fn struct_field(&mut self) -> Result<StructField, ParserError> {
        let (key, value) = self.parse_named_expr()?;
        Ok(StructField { key, value })
    }
}
