use std::str::FromStr;
use std::{
    error::Error,
    iter::{Peekable, Product},
    vec,
};

use crate::{
    expression::{Expr, Func, Op},
    lexer::TokenKind,
};

pub struct Parser<I: Iterator<Item = TokenKind>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = TokenKind>> Parser<I> {
    pub fn new(lexer: Peekable<I>) -> Self {
        Self { tokens: lexer }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_primary()?;

        if let Some(op) = self.tokens.peek() {
            match op {
                TokenKind::Plus => {
                    let rhs = self.parse()?;
                    self.tokens.next();
                    return Ok(Expr::Op(Box::new(lhs), Box::new(rhs), Op::Add));
                }
                TokenKind::Minus => {
                    self.tokens.next();
                    let rhs = self.parse()?;

                    return Ok(Expr::Op(Box::new(lhs), Box::new(rhs), Op::Sub));
                }
                TokenKind::Multiply => {
                    self.tokens.next();
                    let rhs = self.parse()?;

                    return Ok(Expr::Op(Box::new(lhs), Box::new(rhs), Op::Mult));
                }
                TokenKind::Divider => {
                    self.tokens.next();
                    let rhs = self.parse()?;

                    return Ok(Expr::Op(Box::new(lhs), Box::new(rhs), Op::Frac));
                }
                TokenKind::Power => {
                    self.tokens.next();
                    let rhs = self.parse()?;

                    return Ok(Expr::Op(Box::new(lhs), Box::new(rhs), Op::Pow));
                }
                TokenKind::Equals => {
                    self.tokens.next();
                    let rhs = self.parse()?;
                    return Ok(Expr::Op(Box::new(lhs), Box::new(rhs), Op::Equals));
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
                TokenKind::Number(a) => Ok(Expr::Val(a)),
                TokenKind::Identifier(a) if a.len() == 1 => Ok(Expr::Sym(a.as_bytes()[0] as char)),
                TokenKind::Identifier(a) => {
                    if let Some(TokenKind::ParenOpen) = self.tokens.next() {
                        let args = self.parse_args()?;
                        let a = a.try_into()?;
                        Ok(Expr::Func(a, args))
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
