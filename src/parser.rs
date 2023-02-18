use std::str::FromStr;
use std::{
    error::Error,
    iter::{Peekable, Product},
    vec,
};

use crate::expression::UnaryOp;
use crate::{
    expression::{BinaryOp, Expr, Func},
    lexer::TokenKind,
};

pub struct Parser<I: Iterator<Item = TokenKind>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = TokenKind>> Parser<I> {
    pub fn new(lexer: Peekable<I>) -> Self {
        Self { tokens: lexer }
    }

    // TODO: Operator Precedence Lol
    pub fn parse(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_primary()?;

        if let Some(op) = self.tokens.peek() {
            match op {
                TokenKind::Plus => {
                    self.tokens.next();
                    let rhs = self.parse()?;
                    return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Add));
                }
                TokenKind::Minus => {
                    self.tokens.next();
                    let rhs = self.parse()?;
                    return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Sub));
                }
                TokenKind::Multiply => {
                    self.tokens.next();
                    let rhs = self.parse()?;

                    return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Mult));
                }
                TokenKind::Divider => {
                    self.tokens.next();
                    let rhs = self.parse()?;

                    return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Frac));
                }
                TokenKind::Power => {
                    self.tokens.next();
                    let rhs = self.parse()?;

                    return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Pow));
                }
                TokenKind::Equals => {
                    self.tokens.next();
                    let rhs = self.parse()?;
                    return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Equals));
                }
                _ => Ok(lhs),
            }
        } else {
            Ok(lhs)
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        if let Some(primary) = self.tokens.next() {
            match primary {
                TokenKind::Minus => Ok(Expr::Unary(Box::new(self.parse()?), UnaryOp::Sub)),
                TokenKind::Number(a) => Ok(Expr::Val(a)),
                TokenKind::Identifier(a) if a.len() == 1 => Ok(Expr::Sym(a.as_bytes()[0] as char)),
                TokenKind::ParenOpen => {
                    let expr = self.parse()?;
                    let next = self.tokens.next();
                    if Some(TokenKind::ParenClose) == next {
                        Ok(expr)
                    } else {
                        Err(format!("ERROR: Expected ) but got {:?}", next))
                    }
                }
                TokenKind::ParenClose => Err(format!("ERROR: A Primary Cannot Start With (")),
                TokenKind::Identifier(a) => {
                    if let Some(TokenKind::ParenOpen) = self.tokens.next() {
                        let args = self.parse_args()?;
                        let a: Func = a.try_into()?;
                        if a.nr_of_args() != args.len() {
                            Err(format!(
                                "ERROR: Expected {} Arguments but {} Were Provided",
                                a.nr_of_args(),
                                args.len()
                            ))
                        } else {
                            Ok(Expr::Func(a, args))
                        }
                    } else {
                        Err(format!("ERROR: Expected Arguments After Keyword {}", a))
                    }
                }

                other => Err(format!("ERROR: Could Not Parse The Token : {:?}", other)),
            }
        } else {
            Err(format!("ERROR: Unexpected End Of Line"))
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut output = vec![];
        output.push(self.parse()?);

        while let Some(TokenKind::Comma) = self.tokens.peek() {
            self.tokens.next();
            output.push(self.parse()?);
        }

        if let Some(TokenKind::ParenClose) = self.tokens.next() {
            return Ok(output);
        } else {
            todo!()
        }
    }
}
