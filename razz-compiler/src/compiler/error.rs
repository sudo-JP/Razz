use crate::{lexer::error::LexError, parser::error::ParserError, semantic::error::SemanticError};


#[derive(Debug)]
pub enum CompilerError {
    Lexer(Vec<LexError>),
    Parser(Vec<ParserError>), 
    SemanticAnalysis(Vec<SemanticError>),
    IR,
    Codegen,
}
