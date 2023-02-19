use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Func {
    Sqrt,
    Sin,
    Cos,
}

impl Func {
    pub fn nr_of_args(&self) -> usize {
        match &self {
            Func::Sqrt => 1,
            Func::Sin => 1,
            Func::Cos => 1,
        }
    }
}

impl TryFrom<String> for Func {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "sqrt" => Ok(Func::Sqrt),
            "sin" => Ok(Func::Sin),
            "cos" => Ok(Func::Cos),
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

impl BinaryOp {
    pub fn precedence(&self) -> usize {
        match &self {
            BinaryOp::Add => 1,
            BinaryOp::Sub => 1,
            BinaryOp::Mult => 2,
            BinaryOp::Frac => 2,
            BinaryOp::Pow => 2,
            BinaryOp::Equals => 3,
        }
    }
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
    Dots(),
}

impl Expr {
    pub fn is_val(&self) -> bool {
        match &self {
            Expr::Val(_) => true,
            _ => false,
        }
    }
}

pub fn evaulate(expr: &Expr) -> String {
    match expr {
        Expr::Binary(a, b, op) => {
            let a_str = evaulate(a);
            let b_str = evaulate(b);
            match op {
                BinaryOp::Add => {
                    format!("{{{}}} + {{{}}}", a_str, b_str)
                }
                BinaryOp::Sub => {
                    format!("{{{}}} - {{{}}}", a_str, b_str)
                }
                BinaryOp::Mult => {
                    if a.is_val() && b.is_val() {
                        format!("{{{}}} * {{{}}}", a_str, b_str)
                    } else {
                        format!("{{{}}}{{{}}}", a_str, b_str)
                    }
                }
                BinaryOp::Frac => {
                    format!("\\frac{{{}}} {{{}}}", a_str, b_str)
                }
                BinaryOp::Pow => {
                    format!("{{{}}} ^ {{{}}}", a_str, b_str)
                }
                BinaryOp::Equals => {
                    format!("{{{}}} = {{{}}}", a_str, b_str)
                }
            }
        }
        Expr::Unary(a, op) => {
            let a = evaulate(a);
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

                format!("\\sqrt{{{}}}", evaulate(&args[0]))
            }
            Func::Sin => {
                assert_eq!(
                    args.len(),
                    1,
                    "ERROR: Square Root Accepts One Argument, You Supplied {}",
                    args.len()
                );

                format!("\\sin({{{}}})", evaulate(&args[0]))
            }
            Func::Cos => {
                assert_eq!(
                    args.len(),
                    1,
                    "ERROR: Square Root Accepts One Argument, You Supplied {}",
                    args.len()
                );

                format!("\\cos({{{}}})", evaulate(&args[0]))
            }
        },
        Expr::Dots() => "{{{\\dots}}}".to_string(),
    }
}
