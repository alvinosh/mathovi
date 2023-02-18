use clap::Parser;

#[derive(Parser)]
#[command(name = "Mathovi")]
#[command(author = "Alvinosh. <alvihysa89@gmail.com>")]
#[command(version = "0.2")]
#[command(about = "Convert text to math PNG-s", long_about = None)]
pub struct Cli {
    /// Name of the input file
    #[arg(short, long)]
    pub input: String,

    /// Name Of the output file
    #[arg(short, long)]
    pub output: String,
}
