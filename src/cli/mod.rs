use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub output: String,

    #[arg(short, long)]
    pub width: i32, 

    #[arg(long)]
    pub height: i32,

    //#[arg(short, long, default_value = "ppm")]
    //pub render: String,
    #[arg(long, help = "Camera position x", default_value_t)]
    pub cx: f64, 

    #[arg(long, help = "Camera position y", default_value_t)]
    pub cy: f64, 

    #[arg(long, help = "Camera position z", default_value_t)]
    pub cz: f64,

    // TODO: Add viewport width and height, check 
    // either or then calculate based on aspect ratio 
}
