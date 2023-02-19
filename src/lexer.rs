use core::panic;
use std::{iter::Peekable, str::Chars};

// !TODO : UNIT TESTS

#[derive(Debug, PartialEq)]
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
    End,
    Comma,
}

impl TokenKind {
    pub fn takes_precedence(&self, precidence: usize) -> bool {
        match &self {
            TokenKind::Plus => precidence == 1,
            TokenKind::Minus => precidence == 1,
            TokenKind::Multiply => precidence == 2,
            TokenKind::Divider => precidence == 2,
            TokenKind::Power => precidence == 3,
            TokenKind::Equals => precidence == 0,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new<S: AsRef<str>>(text: &'a S) -> Self {
        return Self {
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

impl<'a> Iterator for Lexer<'a> {
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
            ';' => TokenKind::End,
            ',' => TokenKind::Comma,
            c @ ('_' | 'a'..='z' | 'A'..='Z') => self.parse_ident(c),
            c @ '0'..='9' => self.parse_num(c),
            _ => panic!("UNDEFINED TOKEN : {}", current),
        };

        return Some(token);
    }
}

#[test]
fn it_works() {
    let str = "a = sqrt(5 * x, hello()) / sin((1/2) * x);";
    let tokens: Vec<TokenKind> = Lexer::new(&str).collect();

    assert_eq!(
        tokens,
        vec![
            TokenKind::Identifier("a".to_string()),
            TokenKind::Equals,
            TokenKind::Identifier("sqrt".to_string()),
            TokenKind::ParenOpen,
            TokenKind::Number(5.0),
            TokenKind::Multiply,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Comma,
            TokenKind::Identifier("hello".to_string()),
            TokenKind::ParenOpen,
            TokenKind::ParenClose,
            TokenKind::ParenClose,
            TokenKind::Divider,
            TokenKind::Identifier("sin".to_string()),
            TokenKind::ParenOpen,
            TokenKind::ParenOpen,
            TokenKind::Number(1.0),
            TokenKind::Divider,
            TokenKind::Number(2.0),
            TokenKind::ParenClose,
            TokenKind::Multiply,
            TokenKind::Identifier("x".to_string()),
            TokenKind::ParenClose,
            TokenKind::End,
        ]
    );
}
