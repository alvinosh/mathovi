use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Func {
    Sqrt,
}

impl TryFrom<String> for Func {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "sqrt" => Ok(Func::Sqrt),
            _ => Err("Could Not Parse".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Frac,
    Pow,
    Equals,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Op(Box<Expr>, Box<Expr>, Op),
    Sym(char),
    Val(f64),
    Func(Func, Vec<Expr>),
}

pub fn evaulate(expr: Expr) -> String {
    match expr {
        Expr::Op(a, b, op) => {
            let a = evaulate(*a);
            let b = evaulate(*b);
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
                println!("ARGS : {:?}", args);

                assert_eq!(
                    args.len(),
                    1,
                    "ERROR: Square Root Accepts One Argument, You Supplied {}",
                    args.len()
                );

                format!("\\sqrt{{{}}}", evaulate(args[0].clone()))
            }
        },
    }
}
