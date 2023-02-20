use crate::error::Error;

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
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "sqrt" => Ok(Func::Sqrt),
            "sin" => Ok(Func::Sin),
            "cos" => Ok(Func::Cos),
            a => Err(Error::UnexpectedIdent {
                line: 0,
                col: 0,
                ident: a.to_string(),
            }),
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
                    let mut output = String::new();
                    let a_weak = match **a {
                        Expr::Binary(_, _, BinaryOp::Add) => true,
                        Expr::Binary(_, _, BinaryOp::Sub) => true,
                        _ => false,
                    };
                    if a_weak {
                        output.push_str(&format!("{{({})}}", a_str));
                    } else {
                        output.push_str(&format!("{{{}}}", a_str));
                    }

                    if a.is_val() && b.is_val() {
                        output.push_str("*");
                    }
                    let b_weak = match **b {
                        Expr::Binary(_, _, BinaryOp::Add) => true,
                        Expr::Binary(_, _, BinaryOp::Sub) => true,
                        _ => false,
                    };
                    if b_weak {
                        output.push_str(&format!("{{({})}}", b_str));
                    } else {
                        output.push_str(&format!("{{{}}}", b_str));
                    }
                    output
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
