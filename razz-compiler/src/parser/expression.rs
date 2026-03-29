use crate::{ast::{expression::{Arg, BinOpKind, Expr, SpecificType, StructField, UnOpKind}, Spanned}, 
    lexer::tokens::TokenKind, parser::error::{ParserError, ParserErrorKind}};
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
        let mut node = self.function_call()?.node; 
        let rules = [TokenKind::Arrow];
        let span = self.peek().span;
        println!("function call: {:?}", node);
        println!("peek: {:?}", self.peek().kind);

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
    fn function_call(&mut self) -> Result<Spanned<Expr>, ParserError> {
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

    /// Used to parse 
    /// IDENT ":" Expr ; 
    fn parse_named_expr(&mut self) -> Result<(String, Expr), ParserError> {
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
        let token = self.peek();
        let span = token.span;
        let kind = token.kind.clone();

        let node = match kind {
            TokenKind::StringLit(s) => { 
                self.advance();
                Expr::Constant(Literal::String(s))
            },
            TokenKind::IntLit(i) => {
                self.advance();
                Expr::Constant(Literal::Int(i))
            }, 
            TokenKind::FloatLit(f) => {
                self.advance();
                Expr::Constant(Literal::Float(f))
            }, 
            TokenKind::BoolLit(b) => {
                self.advance();
                Expr::Constant(Literal::Bool(b))
            }, 
            TokenKind::NullLit => {
                self.advance();
                Expr::Constant(Literal::Null)
            },
            TokenKind::Get => self.get_request()?,
            TokenKind::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(&TokenKind::RParen)?;
                expr.node
            }, 
            TokenKind::Ident(s) => {
                // Either an identifier or struct literal, lookahead by 2
                if matches!(self.peek_next().kind, TokenKind::LBrace) {
                    self.struct_literal()?
                }
                else { 
                    self.advance();
                    Expr::Identifier(s) 
                }
            },
            TokenKind::Vec3 
            | TokenKind::Point3
            | TokenKind::Color
            | TokenKind::Background 
            | TokenKind::Camera 
            | TokenKind::Output 
            | TokenKind::Sphere 
            | TokenKind::Image 
            => self.struct_literal()?,
            _ => { return Err(self.error(token)); }
        };

        Ok(Spanned{ node, span })
    }

    /// GET_Request ::= "GET" Endpoint ; 
    fn get_request(&mut self) -> Result<Expr, ParserError> {
        self.advance();
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
        let token = self.peek();
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
        self.advance(); 
        Ok(ty)
    }

    /// StructFields ::= [ StructField { "," StructField } ]
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
