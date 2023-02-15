use std::{fmt::format, vec};

use tokenizer::Tokenizer;

mod tokenizer;

#[derive(Clone, Copy, Debug)]
enum Func {
    Sqrt,
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mult,
    Frac,
    Pow,
    Equals,
}

#[derive(Clone, Debug)]
enum Expr {
    Op(Box<Expr>, Box<Expr>, Op),
    Sym(char),
    Val(f64),
    Func(Func, Vec<Expr>),
}

fn parse(expr: Expr) -> String {
    match expr {
        Expr::Op(a, b, op) => {
            let a = parse(*a);
            let b = parse(*b);
            match op {
                Op::Add => {
                    format!("{{{}}} + {{{}}}", a, b)
                }
                Op::Sub => {
                    format!("{{{}}} - {{{}}}", a, b)
                }
                Op::Mult => {
                    format!("{{{}}} * {{{}}}", a, b)
                }
                Op::Frac => {
                    format!("\\frac{{{}}} {{{}}}", a, b)
                }
                Op::Pow => {
                    format!("{{{}}} ^ {{{}}}", a, b)
                }
                Op::Equals => {
                    format!("{{{}}} = {{{}}}", a, b)
                }
            }
        }
        Expr::Sym(c) => c.to_string(),
        Expr::Val(v) => v.to_string(),
        Expr::Func(func, args) => match func {
            Func::Sqrt => {
                assert_eq!(
                    args.len(),
                    1,
                    "ERROR: Square Root Accepts One Argument, You Supplied {}",
                    args.len()
                );

                format!("\\sqrt{{{}}}", parse(args[0].clone()))
            }
        },
    }
}

fn main() {
    // a = b
    let AST: Vec<Expr> = vec![
        Expr::Op(
            Box::new(Expr::Sym('a')),
            Box::new(Expr::Sym('b')),
            Op::Equals,
        ),
        Expr::Op(
            Box::new(Expr::Sym('a')),
            Box::new(Expr::Op(
                Box::new(Expr::Val(-3.0)),
                Box::new(Expr::Val(5.0)),
                Op::Pow,
            )),
            Op::Equals,
        ),
        Expr::Op(
            Box::new(Expr::Sym('b')),
            Box::new(Expr::Op(
                Box::new(Expr::Op(
                    Box::new(Expr::Val(5.0)),
                    Box::new(Expr::Val(23.0)),
                    Op::Pow,
                )),
                Box::new(Expr::Op(
                    Box::new(Expr::Val(5.0)),
                    Box::new(Expr::Val(21.0)),
                    Op::Pow,
                )),
                Op::Frac,
            )),
            Op::Equals,
        ),
        Expr::Op(
            Box::new(Expr::Sym('b')),
            Box::new(Expr::Func(
                Func::Sqrt,
                vec![Expr::Op(
                    Box::new(Expr::Val(5.0)),
                    Box::new(Expr::Val(21.0)),
                    Op::Pow,
                )],
            )),
            Op::Equals,
        ),
    ];

    let mut string = String::new();
    for expr in AST {
        string.push_str(&parse(expr));
        string.push_str("\\\\ ");
    }

    println!("{}", string);
}
