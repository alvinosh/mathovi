#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    fmt::format,
    io::{self, BufRead, BufReader, Write},
    vec,
};

use clap::Parser;
use cli_args::Cli;
use lexer::Lexer;

mod cli_args;
mod expression;
mod lexer;
mod parser;

fn main() {
    let args = Cli::parse();
    let input_string = std::fs::read_to_string(args.input).unwrap();
    let lexer = Lexer::new(&input_string);
    let mut parser = parser::Parser::new(lexer.peekable());
    let tex = expression::evaulate(parser.parse().unwrap());
    let img_url = get_tex_url(tex);

    let url = format!("http://latex2png.com{}", img_url);

    let mut bytes = Vec::new();
    let res = http_req::request::get(url, &mut bytes).unwrap();
    let bytes: &[u8] = &bytes[..];

    let mut f = std::fs::File::create(args.output).unwrap();
    f.write_all(bytes).unwrap();
}

fn get_tex_url(tex: String) -> String {
    let req_body = format!(
        "
    {{
        \"auth\": {{
            \"user\": \"guest\",
            \"password\": \"guest\"
        }},
        \"latex\": {:?},
        \"resolution\": 600,
        \"color\": \"000000\"
    }}
    ",
        tex
    );

    let req_body = req_body.as_bytes();

    let mut writer = Vec::new();

    let res =
        http_req::request::post("http://latex2png.com/api/convert", req_body, &mut writer).unwrap();

    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8(writer.clone()).unwrap()).unwrap();
    let mut output = v.get("url").unwrap().to_string();
    output.pop();
    output.remove(0);

    output
}
