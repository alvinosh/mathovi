use crate::lexer::{Token, TokenKind};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(
        "ERROR at {}:{} : Expected \"{}\" , Found \"{}\"",
        found.line,
        found.col,
        print_vec_tokens(expected),
        found.kind
    )]
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: Token,
    },

    #[error("ERROR at {}:{} : Unexpected Identifier \"{}\"", line, col, ident)]
    UnexpectedIdent {
        line: usize,
        col: usize,
        ident: String,
    },

    #[error("ERROR : Unexpected End Of File")]
    UnexpectedEOF,

    #[error(
        "ERROR at {}:{} : Unexpected Number Of Argumentns, Expected  \"{}\", Found  \"{}\"",
        line,
        col,
        expected,
        found
    )]
    WrongArguments {
        line: usize,
        col: usize,
        found: usize,
        expected: usize,
    },
}

fn print_vec_tokens(vec: &Vec<TokenKind>) -> String {
    let mut ouptut = String::new();
    for token in vec {
        ouptut.push_str(&format!("{}", token));
    }
    ouptut
}
