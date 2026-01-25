use super::{Error, Rational};

#[derive(Debug, Clone)]
pub enum Token {
    Number(Rational),
    Func(String),
    Var(String),
    Plus,
    Minus,
    Times,
    Slash,
    RParen,
    LParen,
    Eof,
}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    expr: String,
}

impl Lexer {
    pub fn new(input: &str, vars: Option<&[&str]>, funcs: Option<&[&str]>) -> Result<Self, Error> {
        let filtered = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        let mut parsed = parse_string(&filtered, vars, funcs)?;
        parsed.reverse();

        Ok(Self {
            tokens: parsed,
            expr: filtered,
        })
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    pub fn peek(&self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }
}

fn parse_string(
    str: &str,
    vars: Option<&[&str]>,
    funcs: Option<&[&str]>,
) -> Result<Vec<Token>, Error> {
    let mut res = Vec::new();
    let mut index = 0;

    let funcs = funcs.unwrap_or(&[]);
    let vars = vars.unwrap_or(&[]);

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
            Token::Func(func)
        } else if let Some(var) = next_segment_in(str, &mut index, &vars) {
            Token::Var(var)
        } else {
            match char {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Times,
                '/' => Token::Slash,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '0'..='9' => Token::Number(parse_number(&str, &mut index)?),
                c => return Err(Error::InvalidToken(index, c)),
            }
        };

        if !matches!(token, Token::Number(_) | Token::Var(_) | Token::Func(_)) {
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

pub fn next_segment_in(str: &str, index: &mut usize, itens: &[&str]) -> Option<String> {
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
