use super::Rational;

#[derive(Debug)]
enum Token {
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
    pub fn new(input: &str, vars: Option<&[&str]>, funcs: Option<&[&str]>) -> Self {
        let filtered = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        Self {
            tokens: parse_string(&filtered, vars, funcs),
            expr: filtered,
        }
    }
}

fn parse_string(str: &str, vars: Option<&[&str]>, funcs: Option<&[&str]>) -> Vec<Token> {
    let mut res = Vec::new();
    let mut index = 0;

    let funcs = funcs.unwrap_or(&[]);
    let vars = vars.unwrap_or(&[]);

    if funcs.iter().any(|func| vars.contains(func)) {
        panic!("Funcs and Vars have an element in common");
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
                '0'..='9' => Token::Number(parse_number(&str, &mut index)),
                _ => panic!("Invalid token: {}", char),
            }
        };

        if !matches!(token, Token::Number(_) | Token::Var(_) | Token::Func(_)) {
            index += 1;
        }
        res.push(token);
    }

    res.push(Token::Eof);
    res
}

fn parse_number(str: &str, index: &mut usize) -> Rational {
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
                //Safety: char is always a number ig
                char.to_digit(10).unwrap() as u128
            }
            '.' => {
                if den_mode {
                    panic!("Unable to parse number: Number has two dots");
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

    //TODO: remove the unwrap
    //Safety: den is non-zero
    Rational::new(num.expect("Number not found"), den, false).unwrap()
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
