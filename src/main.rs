#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use lexer::Lexer;
use parser::Parser;

mod expression;
mod lexer;
mod parser;

fn main() {
    let string = "a = sqrt(5 ^ 23) / 3 * 21";
    let lexer = Lexer::new(&string);

    let mut parser = Parser::new(lexer.peekable());

    //println!("{:?}", parser.parse().unwrap());

    println!("{}", expression::evaulate(parser.parse().unwrap()));
}
