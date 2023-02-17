use clap::Parser;

#[derive(Parser)]
#[command(name = "Mathovi")]
#[command(author = "Alvinosh. <alvihysa89@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Convert text to math PNG-s", long_about = None)]
pub struct Cli {
    /// Name of the input_file
    #[arg(short, long)]
    pub input: String,

    /// Number of times to greet
    #[arg(short, long)]
    pub output: String,
}
