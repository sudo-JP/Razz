use std::collections::{HashMap, HashSet};

use clap::ValueEnum;
use crate::ast::{NodeId, Program, TypeKind};
use crate::codegen::rust_codegen::RustCodegen;
use crate::compiler::error::CompilerError;
use crate::ir::hir::hir_statement::HIRProgram;
use crate::ir::hir::hir_structurizer::HIRStructurizer;
use crate::ir::ssa::optimizer::Optimization;
use crate::ir::ssa::optimizer::constant_folding::ConstantFolding;
use crate::ir::ssa::ssa::SSAProgram;
use crate::ir::ssa::ssa_lowerer::SSALowerer;
use crate::lexer::tokens::Token;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::semantic::analyzer::SemanticAnalyzer;


#[derive(ValueEnum, Clone, PartialEq)]
pub enum CompilerStage {
    Lexer, 
    Parser, 
    SemanticAnalysis, 
    SSAIR, 
    HIR,
    Codegen, 
}

#[derive(Debug)]
pub enum CompilerOutput {
    Lexer(Vec<Token>),
    Parser(Program), 
    SemanticAnalysis(HashSet<NodeId>, HashMap<NodeId, TypeKind>),
    SSAIR(SSAProgram),
    HIR(HIRProgram),
    Codegen,
}

pub struct Compiler {
    debug: CompilerStage,
    optimized: bool, 
}

impl Compiler {
    pub fn new(c: CompilerStage, optimized: bool) -> Self {
        Self { debug: c, optimized }
    }

    pub fn compiles(&self, contents: &str, output: Option<String>) -> Result<CompilerOutput, CompilerError> {
        // ============= LEXER =============  
        let lexer = Lexer::new(contents);

        let tokens = lexer.lex()
            .map_err(CompilerError::Lexer)?;

        if matches!(self.debug, CompilerStage::Lexer) {
            return Ok(CompilerOutput::Lexer(tokens));
        }

        // ============= PARSER ============= 
        let parser = Parser::new(tokens);
        let prog = parser.parse()
            .map_err(CompilerError::Parser)?;

        if matches!(self.debug, CompilerStage::Parser) {
            return Ok(CompilerOutput::Parser(prog));
        }

        // ============= SEMANTIC ANALYSIS ============= 
        let mut analyzer = SemanticAnalyzer::new();
        let (mutable_set, type_table) = analyzer.check(&prog)
            .map_err(CompilerError::SemanticAnalysis)?;

        if matches!(self.debug, CompilerStage::SemanticAnalysis) {
            return Ok(CompilerOutput::SemanticAnalysis(mutable_set, type_table));
        }

        //  ============= IR LOWERING (SSA at least) ============= 
        let lowerer = SSALowerer::new(type_table);
        let ssa_program = lowerer.lower(&prog);
        if matches!(self.debug, CompilerStage::SSAIR) {
            return Ok(CompilerOutput::SSAIR(ssa_program));
        }

        /*let mut const_fol = ConstantFolding;
        const_fol.optimize(&mut ssa_program);*/

        //  ============= HIR STRUCTURIZER ============= 
        let structurizer = HIRStructurizer::new();
        let hir_structurized = structurizer.structurize(ssa_program);
        if matches!(self.debug, CompilerStage::HIR) {
            return Ok(CompilerOutput::HIR(hir_structurized));
        }

        //  ============= CODE GEN ============= 
        let mut codegen = RustCodegen::new(format!("{}.rs", output.unwrap_or_else(|| String::from("generated")))).unwrap();
        codegen.generate(hir_structurized);

        Ok(CompilerOutput::Codegen)
    }

}
