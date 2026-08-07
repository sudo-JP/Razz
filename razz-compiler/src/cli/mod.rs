use clap::Parser;

use crate::compiler::compiler::CompilerStage;


#[derive(Parser)]
pub struct Cli {
    pub path: String, 

    #[arg(short, long, value_enum, default_value_t = CompilerStage::Codegen)]
    pub debug: CompilerStage,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long)]
    pub optimized: bool,
}
