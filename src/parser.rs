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
}
