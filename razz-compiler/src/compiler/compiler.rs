use clap::ValueEnum;
use crate::lexer::tokens::Token;

use crate::lexer::lexer::{LexError, Lexer};

#[derive(ValueEnum, Clone, PartialEq)]
pub enum CompilerStage {
    Lexer, 
    Parser, 
    AST, 
    TypeCheck, 
    IR, 
    Codegen, 
}

pub enum CompilerOutput {
    Lexer(Vec<Token>),
    Parser, 
    AST, 
    TypeCheck, 
    IR,
    Codegen,
}

pub enum CompilerError {
    Lexer(Vec<LexError>),
    Parser, 
    AST, 
    TypeCheck, 
    IR,
    Codegen,
}


pub struct Compiler;
impl Compiler {

    pub fn compiles(contents: &str, c: CompilerStage) -> Result<CompilerOutput, CompilerError> {
        // LEXER 
        let lexer = Lexer::new(contents);
        let _lexed = match lexer.lex() {
            Ok(tokens) => {
                if matches!(c, CompilerStage::Lexer) {
                    return Ok(CompilerOutput::Lexer(tokens));
                }
                tokens
            }
            Err(bad_toks) => {
                return Err(CompilerError::Lexer(bad_toks));
            }
        };

        // PARSER 
        Ok(CompilerOutput::Codegen)
    }

}
