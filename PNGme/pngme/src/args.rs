use clap::Parser;

#[derive(Parser)]
#[clap(name = "pngme")]
#[clap(author = "Tyler G.")]
#[clap(version = "1.0")]
#[clap(about = "Encodes and Decodes secret messages in png files using chunks!", long_about = None)]
pub struct Cli {
    #[clap(value_parser)]
    pub command: Option<String>,

    #[clap(value_parser)]
    pub path: Option<String>,

    #[clap(value_parser)]
    pub chunk_type: Option<String>,

    #[clap(value_parser)]
    pub message: Option<String>,

    #[clap(value_parser)]
    pub output: Option<String>
}