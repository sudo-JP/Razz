use std::collections::{HashMap, HashSet};

use clap::ValueEnum;
use crate::ast::{NodeId, Program, TypeKind};
use crate::compiler::error::CompilerError;
use crate::ir::ssa_lowerer::SSALowerer;
use crate::lexer::tokens::Token;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::semantic::analyzer::SemanticAnalyzer;

#[derive(ValueEnum, Clone, PartialEq)]
pub enum CompilerStage {
    Lexer, 
    Parser, 
    SemanticAnalysis, 
    IR, 
    Codegen, 
}

#[derive(Debug)]
pub enum CompilerOutput {
    Lexer(Vec<Token>),
    Parser(Program), 
    SemanticAnalysis(HashSet<NodeId>, HashMap<NodeId, TypeKind>),
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

        // ============= SEMANTIC ANALYSIS ============= 
        let mut analyzer = SemanticAnalyzer::new();
        let (mutable_set, type_table) = analyzer.check(&prog)
            .map_err(CompilerError::SemanticAnalysis)?;

        if matches!(self.flag, CompilerStage::SemanticAnalysis) {
            return Ok(CompilerOutput::SemanticAnalysis(mutable_set, type_table));
        }

        //  ============= IR LOWERING (SSA at least) ============= 
        let lowerer = SSALowerer::new(type_table);
        let _ = lowerer.lower(&prog);

        Ok(CompilerOutput::Codegen)
    }

}
