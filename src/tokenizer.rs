use core::panic;
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum TokenKind {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divider,
    Power,
    ParenOpen,
    ParenClose,
    Equals,
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new<S: AsRef<str>>(text: &'a S) -> Tokenizer<'a> {
        return Tokenizer {
            chars: text.as_ref().chars().peekable(),
        };
    }

    fn parse_ident(&mut self, start: char) -> TokenKind {
        let mut string = String::from(start);
        while let Some(c) = self.chars.peek() {
            if c.is_alphanumeric() {
                string.push(c.clone());
                self.chars.next();
            } else {
                break;
            }
        }
        return TokenKind::Identifier(string);
    }

    fn parse_num(&mut self, start: char) -> TokenKind {
        let mut string = String::from(start);
        while let Some(c) = self.chars.peek() {
            if c.is_numeric() || *c == '.' {
                string.push(c.clone());
                self.chars.next();
            } else {
                break;
            }
        }
        return TokenKind::Number(string.parse().expect("ERROR: Failed To Parse Number "));
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        let current = loop {
            match self.chars.next() {
                None => return None,
                Some(c) if !c.is_whitespace() => break c,
                _ => {}
            }
        };

        let token = match current {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divider,
            '^' => TokenKind::Power,
            '(' => TokenKind::ParenOpen,
            ')' => TokenKind::ParenClose,
            '=' => TokenKind::Equals,
            c @ ('_' | 'a'..='z' | 'A'..='Z') => self.parse_ident(c),
            c @ '0'..='9' => self.parse_num(c),
            _ => panic!("UNDEFINED TOKEN : {}", current),
        };

        return Some(token);
    }
}
