use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub output: String,

    //#[arg(short, long, default_value = "ppm")]
    //pub render: String,
}
