use crate::{ast::{expression::{Arg, BinOp, BinOpKind, Expr, ExprKind, StructField, UnOp, UnOpKind}, Spanned, SpecificType}, 
    common::Span, lexer::tokens::TokenKind, 
    parser::error::{ParserError, ParserErrorKind}};
use crate::ast::expression::Literal;
use super::parser::Parser;

impl Parser {
    /// GRAMMAR PARSER
    /// Each of these functions are a nonterminal
    /// expression ::= logic_or ;
    pub(in crate::parser) fn expression(&mut self) -> Result<Expr, ParserError> {
        self.logic_or()
    }

    /// logic_or ::= logic_and { "||" logic_and } ; 
    fn logic_or(&mut self) -> Result<Expr, ParserError> {
        let mut node = self.logic_and()?;
        let rules = [TokenKind::Or];
        let start = node.span.start;

        while self.match_token(&rules) {
            let op = BinOp {
                node: BinOpKind::Or, 
                span: self.previous().span,
            };
            let logic_and = self.logic_and()?;
            let end = logic_and.span.end;
            let right = Box::new(logic_and);
            let left = Box::new(node);
            let kind = ExprKind::BinOp{left, op, right};

            let span = Span {start, end};
            node = Expr {
                id: self.next_id(),
                kind,
                span,
            };

        }

        Ok(node)
    }

    /// logic_and ::= equality { "&&" equality } ; 
    fn logic_and(&mut self) -> Result<Expr, ParserError> {
        let mut node = self.equality()?;
        let start = node.span.start;
        let rules = [TokenKind::And];

        while self.match_token(&rules) {
            let op = BinOp {
                node: BinOpKind::And, 
                span: self.previous().span,
            };
            let equality = self.equality()?;
            let end = equality.span.end;
            let right = Box::new(equality);
            let left = Box::new(node);
            let kind = ExprKind::BinOp {left, op, right};
            let span = Span{start, end};
            node = Expr{ 
                id: self.next_id(), 
                kind, 
                span,
            };
        }

        Ok(node)
    }

    /// equality ::= comparison { ("==" | "!=") comparison } ;
    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut node = self.comparison()?;
        let rules = [TokenKind::Eq, TokenKind::Neq];
        let start = node.span.start;
        let mut end = node.span.end; 

        while self.match_token(&rules) {
            let op_kind = match self.previous().kind {
                TokenKind::Eq => BinOpKind::Eq,
                TokenKind::Neq => BinOpKind::Neq,
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };
            let op = BinOp{
               node: op_kind, 
               span: self.previous().span,
            };
            let comparison = self.comparison()?;
            end = comparison.span.end;
            let right = Box::new(comparison);
            let left = Box::new(node);
            
            let kind = ExprKind::BinOp{left, op, right};

            let span = Span{start, end};
            node = Expr{
                id: self.next_id(), 
                kind, 
                span,
            }
        }
        Ok(node)
    }

    /// comparison ::= term { ("<" | "<=" | ">" | ">=") term } ; 
    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut node = self.term()?;
        let rules = [
            TokenKind::Lt, 
            TokenKind::Le, 
            TokenKind::Gt,
            TokenKind::Ge,
        ];
        let start = node.span.start;
        let mut end = start;

        while self.match_token(&rules) {
            let op_kind = match self.previous().kind {
                TokenKind::Lt => BinOpKind::Lt, 
                TokenKind::Le => BinOpKind::Le, 
                TokenKind::Gt => BinOpKind::Gt, 
                TokenKind::Ge => BinOpKind::Ge, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };
            let op = BinOp{
                node: op_kind, 
                span: self.previous().span
            };
            let term = self.term()?;
            end = term.span.end;
            let left = Box::new(node);
            let right = Box::new(term);

            let kind = ExprKind::BinOp{left, op, right};
            let span = Span{start, end};
            node = Expr{
                id: self.next_id(), 
                kind, 
                span,
            };
        }
        Ok(node)
    }

    /// term ::= factor { ("+" | "-") factor } ;
    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut node = self.factor()?;
        let rules = [
            TokenKind::Add,
            TokenKind::Sub,
        ];
        let start = node.span.start;
        let mut end = start;

        while self.match_token(&rules) {
            let op_kind = match self.previous().kind {
                TokenKind::Add => BinOpKind::Add, 
                TokenKind::Sub => BinOpKind::Sub, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };
            let op = BinOp{
                node: op_kind, 
                span: self.previous().span
            };

            let factor = self.factor()?;
            end = factor.span.end;
            let right = Box::new(factor);
            let left = Box::new(node);

            let kind = ExprKind::BinOp{left, op, right};
            let span = Span{start, end};
            node = Expr{
                id: self.next_id(), 
                kind, 
                span,
            };
        }
        Ok(node)
    }

    /// factor ::= unary { ("*" | "/") unary } ; 
    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut node = self.unary()?;
        let rules = [
            TokenKind::Mult,
            TokenKind::Div,
        ];
        let start = node.span.start;
        let mut end = start;

        while self.match_token(&rules) {
            let op_kind = match self.previous().kind {
                TokenKind::Mult => BinOpKind::Mult, 
                TokenKind::Div => BinOpKind::Div, 
                _ => { 
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };
            let op = BinOp{
                node: op_kind, 
                span: self.previous().span
            };

            let unary = self.unary()?;
            end = unary.span.end;
            let right = Box::new(unary);
            let left = Box::new(node);
            
            let kind = ExprKind::BinOp{left, op, right};
            let span = Span{start, end};
            node = Expr{
                id: self.next_id(),
                kind, 
                span,
            };
        }
        Ok(node)
    } 

    /// unary ::= ("!" | "-") unary
    /// | field_access ;
    fn unary(&mut self) -> Result<Expr, ParserError> {
        let rules = [
            TokenKind::Sub, 
            TokenKind::Not,
        ];

        if self.match_token(&rules) {
            let start = self.previous().span.start;
            let mut end = start;
            let un_op_kind = match self.previous().kind {
                TokenKind::Sub => UnOpKind::Minus, 
                TokenKind::Not => UnOpKind::Not, 
                _ => {
                    let kind = ParserErrorKind::InvalidToken(self.previous().kind.clone());
                    return Err(ParserError{span: Span{start, end}, kind}); 
                }
            };
            let op = UnOp{node: un_op_kind, span: self.previous().span};
            let mut node = self.unary()?;
            end = node.span.end;
            let value = Box::new(node); 
            let kind = ExprKind::UnOp { op, value };
            let span = Span{start, end};
            node = Expr{
                id: self.next_id(),
                kind, 
                span,
            };

            return Ok(node);
        }

        self.field_access()
    }

    /// field_access ::= function_call { "->" IDENT } ; 
    fn field_access(&mut self) -> Result<Expr, ParserError> {
        let mut node = self.function_call()?;
        let rules = [TokenKind::Arrow];
        let start = node.span.start;

        while self.match_token(&rules) {
            let ident = self.consume_ident()?;
            let end = self.previous().span.end;

            let obj = Box::new(node);

            let kind = ExprKind::FieldAccess { 
                obj: obj, 
                key: ident,
            };

            let span = Span{start, end};
            node = Expr{
                id: self.next_id(), 
                kind, 
                span
            };
        }
        Ok(node)
    }

    /// function_call ::= IDENT "(" [ Arg { "," Arg } ] ")" 
    /// | primary ;
    fn function_call(&mut self) -> Result<Expr, ParserError> {
        if let TokenKind::Ident(_) = &self.peek().kind
            && self.peek_next().kind == TokenKind::LParen
        {
            let start = self.peek().span.start;
            // Move pass IDENT (
            let name = self.consume_ident()?;
            self.consume(&TokenKind::LParen)?;
            // We are in function
            let args = self.args()?;
            let end = self.consume(&TokenKind::RParen)?.span.end;
            let kind = ExprKind::FunctionCall { name, args };
            let span = Span{start, end};
            let node = Expr{
                id: self.next_id(), 
                kind, 
                span,
            };
            return Ok(node);
        }
        self.primary() 
    }

    /// Args ::= [ Arg { "," Arg } ] ;
    fn args(&mut self) -> Result<Vec<Arg>, ParserError> {
        let mut args: Vec<Arg> = vec![];
        // First one have to manually check 
        let curr = self.peek();
        // [ Arg .. ]
        if matches!(&curr.kind, TokenKind::Ident(_)) 
            && self.peek_next().kind == TokenKind::Colon {
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
    fn parse_named_expr(&mut self) -> Result<(Spanned<String>, Expr), ParserError> {
        let name = self.consume_ident()?;
        self.consume(&TokenKind::Colon)?;
        let expr = self.expression()?;
        Ok((name, expr))
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
    fn primary(&mut self) -> Result<Expr, ParserError> {
        let token = self.advance();
        let start = token.span.start;
        let mut end = start;
        let tok_kind = token.kind.clone();

        let kind = match tok_kind {
            TokenKind::StringLit(s) => ExprKind::Constant(Literal::String(s)),
            TokenKind::IntLit(i) => ExprKind::Constant(Literal::Int(i)), 
            TokenKind::FloatLit(f) => ExprKind::Constant(Literal::Float(f)), 
            TokenKind::BoolLit(b) => ExprKind::Constant(Literal::Bool(b)), 
            TokenKind::NullLit => ExprKind::Constant(Literal::Null),

            TokenKind::Get => self.get_request()?,
            TokenKind::LParen => {
                let expr = self.expression()?;
                end = self.consume(&TokenKind::RParen)?.span.end;
                expr.kind
            }, 
            TokenKind::Ident(s) => ExprKind::Ident(s),
            TokenKind::Vec3 
            | TokenKind::Point3
            | TokenKind::Color
            | TokenKind::Background 
            | TokenKind::Camera 
            | TokenKind::Output 
            | TokenKind::Sphere 
            | TokenKind::Lambertian
            | TokenKind::Dielectric
            | TokenKind::Metal
            | TokenKind::Image 
            => self.struct_literal()?,
            _ => { return Err(self.error_at(Span{start, end}, tok_kind)); }
        };
        let span = Span{start, end};

        Ok(Expr{
            id: self.next_id(),
            kind, 
            span,
        })
    }

    /// GET_Request ::= "GET" Endpoint ; 
    fn get_request(&mut self) -> Result<ExprKind, ParserError> {
        Ok(ExprKind::HTTPRequest(self.consume_endpoint()?))
    }

    /// StructLiteral ::= SpecificType "{" StructFields "}" ; 
    fn struct_literal(&mut self) -> Result<ExprKind, ParserError> {
        let ty = self.specific_type()?;
        self.consume(&TokenKind::LBrace)?;
        let fields = self.struct_fields()?;
        self.consume(&TokenKind::RBrace)?;
        Ok(ExprKind::StructLiteral { ty , fields })
    }

    /// SpecificType ::= "Vec3" 
    /// | "Point3" 
    /// | "Lambertian" 
    /// | "Dielectric" 
    /// | "Metal" 
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
