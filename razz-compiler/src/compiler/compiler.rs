use clap::ValueEnum;
use crate::lexer::tokens::Token;

use crate::lexer::lexer::{LexError, Lexer};

#[derive(ValueEnum, Clone, PartialEq)]
pub enum CompilerStage {
    Lexer, 
    Parser, 
    TypeCheck, 
    IR, 
    Codegen, 
}

pub enum CompilerOutput {
    Lexer(Vec<Token>),
    Parser, 
    TypeCheck, 
    IR,
    Codegen,
}

pub enum CompilerError {
    Lexer(Vec<LexError>),
    Parser, 
    TypeCheck, 
    IR,
    Codegen,
}


pub struct Compiler {
    flag: CompilerStage,
}

impl Compiler {
    pub fn new(c: CompilerStage) -> Self {
        Self { flag: c }
    }

    pub fn compiles(&self, contents: &str) -> Result<CompilerOutput, CompilerError> {
        // LEXER 
        let lexer = Lexer::new(contents);
        let _lexed = match lexer.lex() {
            Ok(tokens) => {
                if matches!(self.flag, CompilerStage::Lexer) {
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
