use crate::lexer::TokenKind;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(
        "ERROR at {}:{} : Expected \"{}\" , Found \"{}\"",
        line,
        col,
        print_vec_tokens(expected),
        print_option_tokens(found)
    )]
    UnexpectedToken {
        line: usize,
        col: usize,
        expected: Vec<TokenKind>,
        found: Option<TokenKind>,
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

fn print_option_tokens(some: &Option<TokenKind>) -> String {
    if let Some(token) = some {
        return String::from(&format!("{}", token));
    } else {
        return String::from("Nothing");
    }
}
