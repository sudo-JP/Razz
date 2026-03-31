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
        todo!()
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
                Type::SpecificType(self.match_specific_type(self.peek())?)
            }
        };
        Ok(ty)
    }

    fn block(&mut self) -> Result<Spanned<Vec<Stmt>>, ParserError> {
        todo!()
    }
}
