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
mod expression;
mod lexer;
mod parser;

const PREFIX: &'static str = r#"
\documentclass{article}

\pagestyle{empty}

\usepackage[a6paper, margin={2cm,2cm},twocolumn, layouthoffset=0pt]{geometry}

\usepackage[utf8]{inputenc}
\usepackage{lmodern}
\usepackage{amssymb}

\begin{document}
"#;

const SUFFIX: &'static str = r#"
\end{document}
"#;

fn main() {
    let args = Cli::parse();
    println!("PARSING FILE...");
    let input_string = std::fs::read_to_string(args.input).unwrap();
    let lexer = Lexer::new(&input_string);
    let mut parser = parser::Parser::new(lexer.peekable());
    let expressions = parser.parse_all().unwrap();
    let mut tex = String::new();

    for e in expressions {
        tex.push_str(&format!("$ {} $ \\\\ \n", expression::evaulate(e)))
    }

    println!("FILE PARSED.");

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

    let mut temp_tex = std::fs::File::create(&temp_file_paths[0]).unwrap();
    temp_tex.write(PREFIX.as_bytes()).unwrap();
    temp_tex.write(tex.as_bytes()).unwrap();
    temp_tex.write(SUFFIX.as_bytes()).unwrap();

    let term_command = format!(
        "latex -output-directory={3} {0}  && dvipng -D 1000 -o {2} {1}",
        &temp_file_paths[0].display(),
        temp_file_paths[1].display(),
        args.output,
        temp_dir.display(),
    );

    println!("RUNNING: LATEX...");
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
    println!("RUNNING: COMPLETE.");
    println!("GENERATED: {}", args.output);

    for file in temp_file_paths {
        std::fs::remove_file(file).unwrap();
    }

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &format!("start {}", args.output)])
            .output()
            .expect("failed to execute process");
    };
}
