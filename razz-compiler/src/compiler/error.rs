use crate::{lexer::error::LexError, parser::error::ParserError};


pub enum CompilerError {
    Lexer(Vec<LexError>),
    Parser(Vec<ParserError>), 
    TypeCheck, 
    IR,
    Codegen,
}
