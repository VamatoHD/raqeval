use super::Rational;

#[derive(Debug)]
enum Token {
    Number(Rational),
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
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            tokens: parse_string(input),
        }
    }
}

fn parse_string(str: &str) -> Vec<Token> {
    let mut res = Vec::new();
    let mut index = 0;

    while index < str.len() {
        let char = match str.chars().nth(index) {
            Some(char) => char,
            None => break, //Should be unreachable
        };
        dbg!(char);

        let token = match char {
            ' ' => {
                index += 1;
                continue;
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '0'..='9' => Token::Number(parse_number(&str, &mut index)),
            _ => panic!("Invalid token: {}", char),
        };

        if !matches!(token, Token::Number(_)) {
            index += 1;
        }
        res.push(token);
    }

    res.push(Token::Eof);
    res
}

fn parse_number(str: &str, index: &mut usize) -> Rational {
    let mut num = None;

    while *index < str.len() {
        let char = match str.chars().nth(*index) {
            Some(char) => char,
            None => break,
        };

        let digit = match char {
            '0'..='9' => {
                //Safety: char is always a number ig
                char.to_string().parse::<u128>().unwrap()
            }
            _ => break,
        };

        if let Some(value) = num {
            num = Some(value * 10 + digit)
        } else {
            num = Some(digit)
        };

        *index += 1
    }

    //TODO: remove the unwrap
    Rational::new(num.expect("Number not found"), 1, false).unwrap()
}
