use clap::ValueEnum;
use crate::ast::Program;
use crate::compiler::error::CompilerError;
use crate::lexer::tokens::Token;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

#[derive(ValueEnum, Clone, PartialEq)]
pub enum CompilerStage {
    Lexer, 
    Parser, 
    TypeCheck, 
    IR, 
    Codegen, 
}

#[derive(Debug)]
pub enum CompilerOutput {
    Lexer(Vec<Token>),
    Parser(Program), 
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
        // ============= LEXER =============  
        let lexer = Lexer::new(contents);

        let tokens = lexer.lex()
            .map_err(CompilerError::Lexer)?;

        if matches!(self.flag, CompilerStage::Lexer) {
            return Ok(CompilerOutput::Lexer(tokens));
        }

        // ============= PARSER ============= 
        let parser = Parser::new(tokens);
        let prog = parser.parse()
            .map_err(CompilerError::Parser)?;

        if matches!(self.flag, CompilerStage::Parser) {
            return Ok(CompilerOutput::Parser(prog));
        }

        Ok(CompilerOutput::Codegen)
    }

}
