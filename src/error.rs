#[derive(Debug)]
pub enum Error {
    DivisionByZero,
    //Lexer
    OverlapElements(Vec<String>),
    InvalidToken(usize, char),
    TwoDots(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DivisionByZero => write!(f, "division by zero"),
            Error::OverlapElements(v) => write!(f, "overlaping element(s): {}", v.join(", ")),
            Error::InvalidToken(i, c) => write!(f, "invalid token at index {}: \"{}\"", i, c),
            Error::TwoDots(i) => write!(f, "invalid dot at index {}", i),
        }
    }
}
