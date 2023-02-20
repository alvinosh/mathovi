#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    fmt::format,
    io::{self, BufRead, BufReader, Write},
    process::Command,
    string, vec,
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

fn run() -> Result<(), error::Error> {
    let args = Cli::parse();
    // println!("PARSING FILE...");

    let input_string = std::fs::read_to_string(args.input)?;
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
    let output_path = std::path::Path::new(&args.output);
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
        args.output,
        temp_dir.display(),
    );

    let output = if cfg!(target_os = "windows") {
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
            .args(["/C", &format!("start {}", args.output)])
            .output()
            .expect("failed to execute process");
    };

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
    } else {
        println!("Progam Exited Sucessfully.")
    }
}
