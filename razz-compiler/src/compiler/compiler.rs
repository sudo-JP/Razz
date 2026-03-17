use clap::ValueEnum;
use crate::lexer::tokens::TokenKind;

use crate::lexer::lexer::Lexer;

#[derive(ValueEnum, Clone, PartialEq)]
pub enum CompileTarget {
    Lexer, 
    Parser, 
    AST, 
    TypeCheck, 
    IR, 
    Codegen, 
}

pub struct Compiler;

impl Compiler {
    pub fn compiles(contents: &str, c: CompileTarget) {
        // LEXER 
        let lexer = Lexer::new(contents);
        let tokens = lexer.scan_tokens();
        if c == CompileTarget::Lexer {
            tokens.iter().for_each(|t| {
                print!("Kind: {:?} ", t.kind);
                match &t.kind {
                    TokenKind::IntLit(p) => print!("Int: {}", p),
                    TokenKind::FloatLit(p) => print!("Float: {}", p),
                    TokenKind::StringLit(p) => print!("String: {}", p),
                    TokenKind::BoolLit(p) => print!("Bool: {}", p),
                    TokenKind::Ident(p) => print!("Ident: {}", p),
                    _ => {}
                }
                println!("");
            });
            return;
        }

        // PARSER 
    }
}
