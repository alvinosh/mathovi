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
$ "#;

const SUFFIX: &'static str = r#" $
\end{document}
"#;

fn main() {
    let args = Cli::parse();
    let input_string = std::fs::read_to_string(args.input).unwrap();
    let lexer = Lexer::new(&input_string);
    let mut parser = parser::Parser::new(lexer.peekable());
    let tex = expression::evaulate(parser.parse().unwrap());

    let temp_file_name = args.output;
    let mut temp = std::fs::File::create(format!("{}.tex", temp_file_name)).unwrap();

    temp.write(PREFIX.as_bytes()).unwrap();
    temp.write(tex.as_bytes()).unwrap();
    temp.write(SUFFIX.as_bytes()).unwrap();

    let term_command = format!("latex {0}.tex && dvipng -D 1000 {0}.dvi", temp_file_name);

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

    println!("Generated {}1.png...", temp_file_name);
}
