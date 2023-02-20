use core::panic;
use std::{fmt::Display, iter::Peekable, str::Chars};

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
    Dot,
    Comma,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenKind::Number(a) => write!(f, "{}", a),
            TokenKind::Identifier(a) => write!(f, "{}", a),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Multiply => write!(f, "*"),
            TokenKind::Divider => write!(f, "/"),
            TokenKind::Power => write!(f, "^"),
            TokenKind::ParenOpen => write!(f, "("),
            TokenKind::ParenClose => write!(f, ")"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::End => write!(f, ";"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Comma => write!(f, ","),
        }
    }
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

    fn parse_ident(&mut self, start: char) -> Option<TokenKind> {
        let mut string = String::from(start);
        while let Some(c) = self.chars.peek() {
            if c.is_alphanumeric() {
                string.push(c.clone());
                self.chars.next();
            } else {
                break;
            }
        }
        return Some(TokenKind::Identifier(string));
    }

    fn parse_num(&mut self, start: char) -> Option<TokenKind> {
        let mut string = String::from(start);
        while let Some(c) = self.chars.peek() {
            if c.is_numeric() || *c == '.' {
                string.push(c.clone());
                self.chars.next();
            } else {
                break;
            }
        }
        return Some(TokenKind::Number(
            string.parse().expect("ERROR: Failed To Parse Number "),
        ));
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

        match current {
            '+' => Some(TokenKind::Plus),
            '-' => Some(TokenKind::Minus),
            '*' => Some(TokenKind::Multiply),
            '/' => Some(TokenKind::Divider),
            '^' => Some(TokenKind::Power),
            '(' => Some(TokenKind::ParenOpen),
            ')' => Some(TokenKind::ParenClose),
            '=' => Some(TokenKind::Equals),
            ';' => Some(TokenKind::End),
            ',' => Some(TokenKind::Comma),
            '.' => Some(TokenKind::Dot),
            c @ ('_' | 'a'..='z' | 'A'..='Z') => self.parse_ident(c),
            c @ '0'..='9' => self.parse_num(c),
            _ => panic!("UNDEFINED TOKEN : {}", current),
        }
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
