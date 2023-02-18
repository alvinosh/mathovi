use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Func {
    Sqrt,
}

impl Func {
    pub fn nr_of_args(&self) -> usize {
        match &self {
            Func::Sqrt => 1,
        }
    }
}

impl TryFrom<String> for Func {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "sqrt" => Ok(Func::Sqrt),
            _ => Err("Function Not Defined".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mult,
    Frac,
    Pow,
    Equals,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    Sub,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Unary(Box<Expr>, UnaryOp),
    Binary(Box<Expr>, Box<Expr>, BinaryOp),
    Sym(char),
    Val(f64),
    Func(Func, Vec<Expr>),
}

pub fn evaulate(expr: Expr) -> String {
    match expr {
        Expr::Binary(a, b, op) => {
            let a = evaulate(*a);
            let b = evaulate(*b);
            match op {
                BinaryOp::Add => {
                    format!("{{{}}} + {{{}}}", a, b)
                }
                BinaryOp::Sub => {
                    format!("{{{}}} - {{{}}}", a, b)
                }
                BinaryOp::Mult => {
                    format!("{{{}}} * {{{}}}", a, b)
                }
                BinaryOp::Frac => {
                    format!("\\frac{{{}}} {{{}}}", a, b)
                }
                BinaryOp::Pow => {
                    format!("{{{}}} ^ {{{}}}", a, b)
                }
                BinaryOp::Equals => {
                    format!("{{{}}} = {{{}}}", a, b)
                }
            }
        }
        Expr::Unary(a, op) => {
            let a = evaulate(*a);
            match op {
                UnaryOp::Sub => {
                    format!("-{{{}}}", a)
                }
            }
        }
        Expr::Sym(c) => format!("{{{}}}", c.to_string()),
        Expr::Val(v) => format!("{{{}}}", v.to_string()),
        Expr::Func(func, args) => match func {
            Func::Sqrt => {
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
