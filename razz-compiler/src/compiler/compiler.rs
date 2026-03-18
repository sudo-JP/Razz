use clap::ValueEnum;
use crate::lexer::tokens::{Token, TokenKind};

use crate::lexer::lexer::{LexErrorKind, Lexer};
use owo_colors::OwoColorize;

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


fn validate_lexer_tokens(lexer: Lexer, c: &CompileTarget) -> Option<Vec<Token>> {
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(bad_toks) => {
            for t in &bad_toks {
                if let LexErrorKind::InvalidChar(c) = t.kind {
                    eprintln!("{} Invalid token {} at line: {}, column: {}", 
                        "Error:".red(), c, t.line, t.col);
                }
            }
            return None; 
        }
    };

    if matches!(c, CompileTarget::Lexer) {
        for t in &tokens {
            print!("Line: {}, Col: {}, Kind: {:?} ", t.line, t.col, t.kind);
            match &t.kind {
                TokenKind::IntLit(p) => print!("Int: {p}"),
                TokenKind::FloatLit(p) => print!("Float: {p}"),
                TokenKind::StringLit(p) => print!("String: {p}"),
                TokenKind::BoolLit(p) => print!("Bool: {p}"),
                TokenKind::Ident(p) => print!("Ident: {p}"),
                _ => {}
            }
            println!();
        }
    }

    Some(tokens)
}

impl Compiler {

    pub fn compiles(contents: &str, c: CompileTarget) {
        // LEXER 
        let lexer = Lexer::new(contents);

        let Some(_tokens) = validate_lexer_tokens(lexer, &c) 
            else { return };

        // PARSER 
        println!("{}", "Finished".green());
    }

}
