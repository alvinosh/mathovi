use std::{
    io::Write,
    process::{Command, ExitCode},
};

use clap::Parser;
use cli_args::Cli;
use lexer::Lexer;

mod cli_args;
mod consts;
mod error;
mod expression;
mod lexer;
mod parser;

// TODO: Add a way to change the color of the background and foreground
// TODO: Add better logging and a better way to track progress
// TODO: Allow for differential and integral equations
// TODO: Make output argument optional
// TODO: Allow for parsing a equation entered inline

fn run() -> Result<(), error::Error> {
    let args = Cli::parse();
    let input_string = if let Some(str) = args.string {
        str
    } else if let Some(input_file) = args.input_file {
        std::fs::read_to_string(input_file)?
    } else {
        unreachable!();
    };

    let lexer = Lexer::new(&input_string);

    let mut parser = parser::Parser::new(lexer.peekable());

    let expressions = parser.parse_all()?;
    let mut tex = String::new();

    for e in expressions {
        tex.push_str(&format!(
            "\\begin{{displaymath}} {} \\end{{displaymath}}\n",
            expression::evaulate(&e)
        ))
    }

    let temp_dir = std::env::temp_dir();
    let output_path = std::path::Path::new(&args.output_file);
    let file_name = output_path
        .file_stem()
        .expect("ERROR: Incorrect Output Path.");

    let temp_file_path = temp_dir.join(file_name);

    let temp_file_paths = [
        temp_file_path.with_extension("tex"),
        temp_file_path.with_extension("dvi"),
        temp_file_path.with_extension("log"),
        temp_file_path.with_extension("aux"),
    ];

    let mut temp_tex = std::fs::File::create(&temp_file_paths[0])?;
    temp_tex.write(consts::PREFIX.as_bytes())?;
    temp_tex.write(tex.as_bytes())?;
    temp_tex.write(consts::SUFFIX.as_bytes())?;

    let term_command = format!(
        "latex -output-directory={3} {0}  && dvipng -D 1000 -o {2} {1}",
        &temp_file_paths[0].display(),
        temp_file_paths[1].display(),
        args.output_file.display(),
        temp_dir.display(),
    );

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &term_command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&term_command)
            .output()
            .expect("failed to execute process")
    };
    for file in temp_file_paths {
        std::fs::remove_file(file)?;
    }

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &format!("start {}", args.output_file.display())])
            .output()
            .expect("failed to execute process");
    };

    Ok(())
}

fn main() -> std::process::ExitCode {
    if let Err(error) = run() {
        eprintln!("{}", error);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
