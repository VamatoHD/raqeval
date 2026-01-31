use super::Token;

#[derive(Debug)]
pub enum Error {
    DivisionByZero,
    //Parser
    InvalidParens,
    InvalidToken(Token),
    AtomExpected(Token),
    //Lexer
    OverlapElements(Vec<String>),
    InvalidCharacter(usize, char),
    TwoDots(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DivisionByZero => write!(f, "division by zero"),
            Error::InvalidParens => write!(f, "Invalid number of parentesis"),
            Error::InvalidToken(t) => write!(f, "Invalid token: {}", t),
            Error::AtomExpected(t) => write!(f, "Atom expected, got: {}", t),
            Error::OverlapElements(v) => write!(f, "overlaping element(s): {}", v.join(", ")),
            Error::InvalidCharacter(i, c) => write!(f, "invalid token at index {}: \"{}\"", i, c),
            Error::TwoDots(i) => write!(f, "invalid dot at index {}", i),
        }
    }
}
