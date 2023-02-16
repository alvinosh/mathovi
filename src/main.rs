use lexer::Lexer;
use parser::Parser;

mod expression;
mod lexer;
mod parser;

fn main() {
    let string = "5 = (5 * 23)/(3 * 21);";
    let lexer = Lexer::new(&string);
    let mut parser = Parser::new(lexer.peekable());
}
