use clap::Parser;

use crate::compiler::compiler::CompilerStage;


#[derive(Parser)]
pub struct Cli {
    pub path: String, 

    #[arg(short, long, value_enum, default_value_t = CompilerStage::Codegen)]
    pub compile: CompilerStage,
}
