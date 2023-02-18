#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{
    fmt::format,
    io::{self, BufRead, BufReader, Write},
    vec,
};

use lexer::Lexer;
use parser::Parser;

mod expression;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).unwrap();
    let mut lines = std::io::BufReader::new(file).lines();
    let first = lines.next().unwrap().unwrap();
    let lexer = Lexer::new(&first);
    let mut parser = Parser::new(lexer.peekable());
    let tex = expression::evaulate(parser.parse().unwrap());

    let request_body = format!(
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

    let mut writer = Vec::new();

    let res = http_req::request::post(
        "http://latex2png.com/api/convert",
        request_body.as_bytes(),
        &mut writer,
    )
    .unwrap();

    let v: serde_json::Value =
        serde_json::from_str(&String::from_utf8(writer.clone()).unwrap()).unwrap();
    let mut a = v.get("url").unwrap().to_string();
    a.pop();
    a.remove(0);
    let url = format!("http://latex2png.com{}", a);

    let mut writer = Vec::new();

    let res = http_req::request::get(url, &mut writer).unwrap();

    let mut f = std::fs::File::create("foo.png").unwrap();
    let wow: &[u8] = &writer[..];

    f.write_all(wow).unwrap();
}
