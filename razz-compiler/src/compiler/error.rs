use crate::{lexer::error::LexError, parser::error::ParserError};


#[derive(Debug)]
pub enum CompilerError {
    Lexer(Vec<LexError>),
    Parser(Vec<ParserError>), 
    SemanticAnalysis,
    IR,
    Codegen,
}
