use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub output: String,

    #[arg(short, long)]
    pub width: i32, 

    #[arg(short, long)]
    pub height: i32,

    //#[arg(short, long, default_value = "ppm")]
    //pub render: String,
}
