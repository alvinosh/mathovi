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

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub col: usize,
    pub size: usize,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize, size: usize) -> Self {
        Self {
            line,
            col,
            kind,
            size,
        }
    }

    pub fn some(kind: TokenKind, line: usize, col: usize, size: usize) -> Option<Self> {
        Some(Self::new(kind, line, col, size))
    }
}

#[derive(Clone)]
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new<S: AsRef<str>>(text: &'a S) -> Self {
        return Self {
            chars: text.as_ref().chars().peekable(),
            line: 0,
            col: 0,
        };
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<char> {
        match self.peek() {
            Some('\n') => {
                self.line += 1;
                self.col = 0;
                self.chars.next()
            }
            _ => {
                self.col += 1;
                self.chars.next()
            }
        }
    }

    fn parse_ident(&mut self, start: char) -> Option<Token> {
        let mut string = String::from(start);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                string.push(c.clone());
                self.next();
            } else {
                break;
            }
        }
        // fixme size
        return Token::some(TokenKind::Identifier(string), self.line, self.col, 0);
    }

    fn parse_num(&mut self, start: char) -> Option<Token> {
        let mut string = String::from(start);
        while let Some(c) = self.peek() {
            if c.is_numeric() || *c == '.' {
                string.push(c.clone());
                self.next();
            } else {
                break;
            }
        }

        // fixme size
        return Token::some(
            TokenKind::Number(string.parse().expect("Unreachable")),
            self.line,
            self.col,
            0,
        );
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let current = loop {
            match self.next() {
                None => return None,
                Some(c) if !c.is_whitespace() => break c,
                _ => {}
            }
        };

        match current {
            '+' => Token::some(TokenKind::Plus, self.line, self.col, 1),
            '-' => Token::some(TokenKind::Minus, self.line, self.col, 1),
            '*' => Token::some(TokenKind::Multiply, self.line, self.col, 1),
            '/' => Token::some(TokenKind::Divider, self.line, self.col, 1),
            '^' => Token::some(TokenKind::Power, self.line, self.col, 1),
            '(' => Token::some(TokenKind::ParenOpen, self.line, self.col, 1),
            ')' => Token::some(TokenKind::ParenClose, self.line, self.col, 1),
            '=' => Token::some(TokenKind::Equals, self.line, self.col, 1),
            ';' => Token::some(TokenKind::End, self.line, self.col, 1),
            ',' => Token::some(TokenKind::Comma, self.line, self.col, 1),
            '.' => Token::some(TokenKind::Dot, self.line, self.col, 1),
            c @ ('_' | 'a'..='z' | 'A'..='Z') => self.parse_ident(c),
            c @ '0'..='9' => self.parse_num(c),
            _ => panic!("UNDEFINED TOKEN : {}", current),
        }
    }
}
