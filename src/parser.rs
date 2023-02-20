use std::{iter::Peekable, vec};

use crate::error::Error;
use crate::expression::UnaryOp;
use crate::{
    expression::{BinaryOp, Expr, Func},
    lexer::TokenKind,
};

const MAX_PRECEDENCE: usize = 4;

pub struct Parser<I: Iterator<Item = TokenKind>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = TokenKind>> Parser<I> {
    pub fn new(lexer: Peekable<I>) -> Self {
        Self { tokens: lexer }
    }
    pub fn parse_all(&mut self) -> Result<Vec<Expr>, Error> {
        let mut output = vec![];
        output.push(self.parse(0)?);

        loop {
            match self.tokens.next() {
                Some(TokenKind::End) => {
                    if let None = self.tokens.peek() {
                        break;
                    }
                    output.push(self.parse(0)?)
                }
                Some(a) => {
                    return Err(Error::UnexpectedToken {
                        line: 0,
                        col: 0,
                        expected: vec![TokenKind::End],
                        found: Some(a),
                    })
                }
                None => break,
            }
        }

        Ok(output)
    }

    fn parse(&mut self, precedence: usize) -> Result<Expr, Error> {
        if precedence >= MAX_PRECEDENCE {
            return self.parse_primary();
        }
        let lhs = self.parse(precedence + 1)?;

        if let Some(op) = self.tokens.peek() {
            if op.takes_precedence(precedence) {
                match op {
                    TokenKind::Plus => {
                        self.tokens.next();
                        let rhs = self.parse(precedence)?;
                        return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Add));
                    }
                    TokenKind::Minus => {
                        self.tokens.next();
                        let rhs = self.parse(precedence)?;
                        return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Sub));
                    }
                    TokenKind::Multiply => {
                        self.tokens.next();
                        let rhs = self.parse(precedence)?;

                        return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Mult));
                    }
                    TokenKind::Divider => {
                        self.tokens.next();
                        let rhs = self.parse(precedence)?;

                        return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Frac));
                    }
                    TokenKind::Power => {
                        self.tokens.next();
                        let rhs = self.parse(precedence)?;

                        return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Pow));
                    }
                    TokenKind::Equals => {
                        self.tokens.next();
                        let rhs = self.parse(precedence)?;
                        return Ok(Expr::Binary(Box::new(lhs), Box::new(rhs), BinaryOp::Equals));
                    }
                    _ => Ok(lhs),
                }
            } else {
                Ok(lhs)
            }
        } else {
            Ok(lhs)
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, Error> {
        if let Some(primary) = self.tokens.next() {
            match primary {
                TokenKind::Dot => {
                    let next2 = (self.tokens.next(), self.tokens.next());
                    match next2 {
                        (Some(TokenKind::Dot), Some(TokenKind::Dot)) => Ok(Expr::Dots()),
                        (Some(TokenKind::Dot), a) => Err(Error::UnexpectedToken {
                            line: 0,
                            col: 0,
                            expected: vec![TokenKind::Dot],
                            found: a,
                        }),
                        (a, _) => Err(Error::UnexpectedToken {
                            line: 0,
                            col: 0,
                            expected: vec![TokenKind::Dot],
                            found: a,
                        }),
                    }
                }
                TokenKind::Minus => Ok(Expr::Unary(Box::new(self.parse(0)?), UnaryOp::Sub)),
                TokenKind::Number(a) => Ok(Expr::Val(a)),
                TokenKind::Identifier(a) if a.len() == 1 => Ok(Expr::Sym(a.as_bytes()[0] as char)),
                TokenKind::ParenOpen => {
                    let expr = self.parse(0)?;
                    let next = self.tokens.next();
                    if Some(TokenKind::ParenClose) == next {
                        Ok(expr)
                    } else {
                        Err(Error::UnexpectedToken {
                            line: 0,
                            col: 0,
                            expected: vec![TokenKind::ParenClose],
                            found: next,
                        })
                    }
                }
                TokenKind::ParenClose => Err(Error::UnexpectedToken {
                    line: 0,
                    col: 0,
                    expected: vec![
                        TokenKind::Dot,
                        TokenKind::Minus,
                        TokenKind::Number(0.0),
                        TokenKind::Identifier("".to_string()),
                        TokenKind::ParenOpen,
                    ],
                    found: Some(TokenKind::ParenClose),
                }),
                TokenKind::Identifier(a) => {
                    let next = self.tokens.next();
                    if let Some(TokenKind::ParenOpen) = next {
                        let args = self.parse_args()?;
                        let a: Func = a.try_into()?;
                        if a.nr_of_args() != args.len() {
                            Err(Error::WrongArguments {
                                line: 0,
                                col: 0,
                                found: args.len(),
                                expected: a.nr_of_args(),
                            })
                        } else {
                            Ok(Expr::Func(a, args))
                        }
                    } else {
                        Err(Error::UnexpectedToken {
                            line: 0,
                            col: 0,
                            expected: vec![TokenKind::ParenOpen],
                            found: next,
                        })
                    }
                }
                other => Err(Error::UnexpectedToken {
                    line: 0,
                    col: 0,
                    expected: vec![],
                    found: Some(other),
                }),
            }
        } else {
            Err(Error::UnexpectedEOF)
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, Error> {
        let mut output = vec![];
        output.push(self.parse(0)?);

        while let Some(TokenKind::Comma) = self.tokens.peek() {
            self.tokens.next();
            output.push(self.parse(0)?);
        }

        let next = self.tokens.next();
        if let Some(TokenKind::ParenClose) = next {
            return Ok(output);
        } else {
            return Err(Error::UnexpectedToken {
                line: 0,
                col: 0,
                expected: vec![TokenKind::ParenClose],
                found: next,
            });
        }
    }
}
