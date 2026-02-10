use crate::{Ctx, Error, Rational};

#[derive(Debug, PartialEq)]
pub enum Assoc {
    Left,
    Right,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Ident {
    Var(String),
    Func(String),
    Unknown(char),
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ident::Func(v) => write!(f, "Func: {}", v),
            Ident::Var(v) => write!(f, "Var: {}", v),
            Ident::Unknown(v) => write!(f, "Unknown: {}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Number(Rational),
    Ident(Ident),
    Op(Op),
    RParen,
    LParen,
    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Ident(i) => write!(f, "{}", i),
            Token::Op(o) => write!(f, "{}", o),
            Token::RParen => write!(f, ")"),
            Token::LParen => write!(f, "("),
            Token::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str, ctx: Option<&Ctx>) -> Result<Self, Error> {
        let filtered = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let mut parsed = parse_string(&filtered, ctx, false)?;
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

pub(crate) fn parse_string(
    str: &str,
    ctx: Option<&Ctx>,
    allow_unknown: bool,
) -> Result<Vec<Token>, Error> {
    let mut res = Vec::new();
    let mut index = 0;

    let (funcs, vars) = match ctx {
        Some(c) => (c.get_funcs_names(), c.get_vars_names()),
        None => (vec![], vec![]),
    };

    let overlap: Vec<_> = funcs
        .iter()
        .filter_map(|func| {
            if vars.contains(func) {
                Some(func.to_string())
            } else {
                None
            }
        })
        .collect();

    if !overlap.is_empty() {
        return Err(Error::OverlapElements(overlap));
    }

    while index < str.len() {
        let char = match str.chars().nth(index) {
            Some(char) => char,
            None => break, //Should be unreachable
        };

        let token = if let Some(func) = next_segment_in(str, &mut index, &funcs) {
            Token::Ident(Ident::Func(func))
        } else if let Some(var) = next_segment_in(str, &mut index, &vars) {
            Token::Ident(Ident::Var(var))
        } else {
            match char {
                '+' => Token::Op(Op::Add),
                '-' => Token::Op(Op::Sub),
                '*' => Token::Op(Op::Mul),
                '/' => Token::Op(Op::Div),
                '^' => Token::Op(Op::Exp),
                '(' => Token::LParen,
                ')' => Token::RParen,
                '0'..='9' => Token::Number(parse_number(&str, &mut index)?),
                ' ' => {
                    index += 1;
                    continue;
                }
                c => {
                    if allow_unknown {
                        Token::Ident(Ident::Unknown(c))
                    } else {
                        return Err(Error::InvalidCharacter(index, c));
                    }
                }
            }
        };

        // Only increase the index if token isn't a number, a var nor a func
        // Since index was already increased
        if !matches!(token, Token::Number(_) | Token::Ident(_))
            || matches!(token, Token::Ident(Ident::Unknown(_)))
        {
            index += 1;
        }
        res.push(token);
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
