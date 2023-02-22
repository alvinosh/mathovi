use std::{iter::Peekable, vec};

use crate::error::Error;
use crate::expression::UnaryOp;
use crate::lexer::TokenKind;
use crate::{
    expression::{BinaryOp, Expr, Func},
    lexer::Token,
};

const MAX_PRECEDENCE: usize = 4;

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(lexer: Peekable<I>) -> Self {
        Self { tokens: lexer }
    }
    pub fn parse_all(&mut self) -> Result<Vec<Expr>, Error> {
        let mut output = vec![];
        output.push(self.parse(0)?);

        loop {
            match self.tokens.next() {
                Some(Token {
                    kind: TokenKind::End,
                    ..
                }) => {
                    if let None = self.tokens.peek() {
                        break;
                    }
                    output.push(self.parse(0)?)
                }
                Some(token) => {
                    return Err(Error::UnexpectedToken {
                        expected: vec![TokenKind::End],
                        found: token,
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
            if op.kind.takes_precedence(precedence) {
                match op.kind {
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
            match primary.kind {
                TokenKind::Dot => {
                    let next2 = (self.tokens.next(), self.tokens.next());
                    match next2 {
                        (
                            Some(Token {
                                kind: TokenKind::Dot,
                                ..
                            }),
                            Some(Token {
                                kind: TokenKind::Dot,
                                ..
                            }),
                        ) => Ok(Expr::Dots()),
                        (
                            Some(Token {
                                kind: TokenKind::Dot,
                                ..
                            }),
                            Some(a),
                        ) => Err(Error::UnexpectedToken {
                            expected: vec![TokenKind::Dot],
                            found: a,
                        }),
                        (Some(a), _) => Err(Error::UnexpectedToken {
                            expected: vec![TokenKind::Dot],
                            found: a,
                        }),
                        (None, _) => Err(Error::UnexpectedEOF),
                    }
                }
                TokenKind::Minus => Ok(Expr::Unary(Box::new(self.parse(0)?), UnaryOp::Sub)),
                TokenKind::Number(a) => Ok(Expr::Val(a)),
                TokenKind::Identifier(a) if a.len() == 1 => Ok(Expr::Sym(a.as_bytes()[0] as char)),
                TokenKind::ParenOpen => {
                    let expr = self.parse(0)?;
                    let next = self.tokens.next();
                    if let Some(token) = next {
                        match token.kind {
                            TokenKind::ParenClose => Ok(expr),
                            _ => Err(Error::UnexpectedToken {
                                expected: vec![TokenKind::ParenClose],
                                found: token,
                            }),
                        }
                    } else {
                        Err(Error::UnexpectedEOF)
                    }
                }
                TokenKind::ParenClose => Err(Error::UnexpectedToken {
                    expected: vec![
                        TokenKind::Dot,
                        TokenKind::Minus,
                        TokenKind::Number(0.0),
                        TokenKind::Identifier("".to_string()),
                        TokenKind::ParenOpen,
                    ],
                    found: primary,
                }),
                TokenKind::Identifier(a) => {
                    let next = self.tokens.next();
                    if let Some(token) = next {
                        match token.kind {
                            TokenKind::ParenOpen => {
                                let args = self.parse_args()?;
                                let a: Func = a.try_into()?;
                                if a.nr_of_args() != args.len() {
                                    Err(Error::WrongArguments {
                                        line: token.line,
                                        col: token.col,
                                        found: args.len(),
                                        expected: a.nr_of_args(),
                                    })
                                } else {
                                    Ok(Expr::Func(a, args))
                                }
                            }
                            _ => Err(Error::UnexpectedToken {
                                expected: vec![TokenKind::ParenOpen],
                                found: token,
                            }),
                        }
                    } else {
                        Err(Error::UnexpectedEOF)
                    }
                }
                _ => Err(Error::UnexpectedToken {
                    expected: vec![],
                    found: primary,
                }),
            }
        } else {
            Err(Error::UnexpectedEOF)
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, Error> {
        let mut output = vec![];
        output.push(self.parse(0)?);

        while let Some(Token {
            kind: TokenKind::Comma,
            ..
        }) = self.tokens.peek()
        {
            self.tokens.next();
            output.push(self.parse(0)?);
        }

        let next = self.tokens.next();
        if let Some(Token {
            kind: TokenKind::ParenClose,
            ..
        }) = next
        {
            return Ok(output);
        } else if let Some(token) = next {
            return Err(Error::UnexpectedToken {
                expected: vec![TokenKind::ParenClose],
                found: token,
            });
        } else {
            return Err(Error::UnexpectedEOF);
        }
    }
}
