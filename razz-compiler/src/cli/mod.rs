use clap::Parser;

use crate::compiler::compiler::CompileTarget;


#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub path: String, 

    #[arg(short, long, value_enum, default_value_t = CompileTarget::Codegen)]
    pub compile: CompileTarget,
}
