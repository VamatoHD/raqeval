use crate::{Error, Rational};

#[derive(Debug, PartialEq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl Op {
    pub fn get_info(&self) -> (usize, Assoc) {
        match self {
            Op::Add | Op::Sub => (1, Assoc::Left),
            Op::Mul | Op::Div => (2, Assoc::Left),
            Op::Exp => (3, Assoc::Right),
        }
    }

    pub fn apply(&self, a: &Rational, b: &Rational) -> Rational {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
            _ => unimplemented!(),
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Exp => write!(f, "^"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Rational),
    String(String),
    Op(Op),
    RParen,
    LParen,
    Comma,
    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "{}", s),
            Token::Op(o) => write!(f, "{}", o),
            Token::RParen => write!(f, ")"),
            Token::LParen => write!(f, "("),
            Token::Comma => write!(f, ","),
            Token::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Result<Self, Error> {
        let filtered = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let mut parsed = parse_string(&filtered)?;
        parsed.reverse();

        Ok(Self { tokens: parsed })
    }

    pub fn from_tokens(tokens: &Vec<Token>) -> Self {
        let mut rev = tokens.clone();
        rev.reverse();
        Self { tokens: rev }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }
}

pub(crate) fn parse_string(str: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < str.len() {
        let char = match str.chars().nth(index) {
            Some(char) => char,
            None => break, //Should be unreachable
        };

        let token = match char {
            ' ' => {
                index += 1;
                continue;
            }
            '+' => Token::Op(Op::Add),
            '-' => Token::Op(Op::Sub),
            '*' => Token::Op(Op::Mul),
            '/' => Token::Op(Op::Div),
            '^' => Token::Op(Op::Exp),
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '0'..='9' => Token::Number(parse_number(&str, &mut index)?),
            //A specific token for char would add extra complexity.
            c => Token::String(c.to_string()),
        };

        // Only increase the index if token isn't a number
        // Since index was already increased
        if !matches!(token, Token::Number(_)) {
            index += 1;
        }
        tokens.push(token);
    }

    let mut res = Vec::with_capacity(tokens.len());
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        let mut buffer = match token {
            Token::String(s) => s,
            _ => {
                res.push(token);
                continue;
            }
        };

        while let Some(next) = iter.peek() {
            match next {
                Token::String(s) => {
                    buffer.push_str(s);
                    iter.next();
                }
                _ => break,
            }
        }

        res.push(Token::String(buffer));
    }

    Ok(res)
}

fn parse_number(str: &str, index: &mut usize) -> Result<Rational, Error> {
    let mut num = None;
    let mut den = 1;
    let mut den_mode = false;

    while *index < str.len() {
        let char = match str.chars().nth(*index) {
            Some(char) => char,
            None => break,
        };

        let digit = match char {
            '0'..='9' => {
                //Safety: char is always a number
                char.to_digit(10).unwrap() as u128
            }
            '.' => {
                if den_mode {
                    return Err(Error::TwoDots(*index));
                };
                den_mode = true;
                *index += 1;
                continue;
            }
            _ => break,
        };

        num = match num {
            Some(value) => Some(value * 10 + digit),
            None => Some(digit),
        };

        if den_mode {
            den *= 10
        }

        *index += 1
    }

    //Safety: den is non-zero
    Ok(Rational::new(num.expect("Number not found"), den, false)?)
}

fn next_segment_in(str: &str, index: &mut usize, itens: &[&str]) -> Option<String> {
    if str.len() == 0 || itens.len() == 0 {
        return None;
    };

    for item in itens {
        if str.get(*index..*index + item.len()) == Some(item) {
            *index += item.len();
            return Some(item.to_string());
        }
    }

    None
}
