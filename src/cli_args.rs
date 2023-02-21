use clap::Parser;

#[derive(Parser)]
#[command(name = "Mathovi")]
#[command(author = "Alvinosh. <alvihysa89@gmail.com>")]
#[command(version = "0.2")]
#[command(about = "Convert text to math PNG-s", long_about = None)]
#[clap(group(
    clap::ArgGroup::new("input")
        .required(true)
        .args(&["input_file", "string"]),
))]
pub struct Cli {
    /// Name of the input file
    #[arg(short, long, value_parser = clap::value_parser!(std::path::PathBuf), value_name = "INPUT")]
    pub input_file: Option<std::path::PathBuf>,

    /// One Liner String
    #[arg(short, long, value_name = "INPUT")]
    pub string: Option<String>,

    /// Name Of the output file
    #[arg(short, long, value_parser = clap::value_parser!(std::path::PathBuf))]
    pub output_file: std::path::PathBuf,
}
