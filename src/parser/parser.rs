use super::capture;
use crate::{Error, Expr, Func, Rational, lexer::*};

fn compute_atom(lexer: &mut Lexer) -> Result<Expr, Error> {
    match lexer.next() {
        Token::LParen => {
            let expr = compute_expr(lexer, 1)?;
            if matches!(lexer.next(), Token::RParen) {
                Ok(expr)
            } else {
                Err(Error::InvalidParens)
            }
        }

        Token::String(s) => {
            if lexer.peek() == Token::LParen {
                //A function call
                lexer.next(); //Consume "("

                let expr = compute_expr(lexer, 1)?;

                if lexer.next() == Token::RParen {
                    Ok(Expr::Call {
                        func: s,
                        arg: Box::new(expr),
                    })
                } else {
                    Err(Error::InvalidParens)
                }
            } else {
                //A variable
                Ok(Expr::Var(s))
            }
        }

        Token::Number(n) => Ok(Expr::Const(n)),
        t => Err(Error::AtomExpected(t)),
    }
}

fn compute_expr(lexer: &mut Lexer, min_prec: usize) -> Result<Expr, Error> {
    let mut atom_lhs = compute_atom(lexer)?;

    loop {
        // Break out of the loop if:
        // - Precedence is lower
        // - No more tokens left
        // - Isn't a binary operation
        let op = match lexer.peek() {
            Token::Op(op) => {
                let (prec, _) = op.get_info();
                if prec < min_prec {
                    break;
                };
                op.clone()
            }
            Token::Eof => break,
            _ => break,
        };

        //Consume the next token
        lexer.next();

        let (prec, assoc) = op.get_info();
        let next_min_prec = if assoc == Assoc::Left { prec + 1 } else { prec };

        let atom_rhs = compute_expr(lexer, next_min_prec)?;

        atom_lhs = Expr::Infix {
            lhs: Box::new(atom_lhs),
            op,
            rhs: Box::new(atom_rhs),
        };
    }

    Ok(atom_lhs)
}

pub fn parse(input: &str) -> Result<Expr, Error> {
    let mut lexer = Lexer::new(input)?;
    compute_expr(&mut lexer, 1)
}

pub fn parse_func(input: &str) -> Result<Func, Error> {
    let (lhs, rhs) = input
        .split_once("=")
        .ok_or_else(|| Error::InvalidFunc("no \"=\" found".to_string()))?;

    let mut lhs_tokens = parse_string(lhs)?;

    let items = capture!(
        lhs_tokens,
        [Token::String(_) | Token::Number(_)],
        Token::LParen,
        [Token::String(_)],
        Token::RParen
    )
    .ok_or_else(|| Error::InvalidFunc("invalid function signature".to_string()))?;

    let func_name = items[0]
        .iter()
        .try_fold(String::new(), |mut acc, value| match value {
            Token::String(s) => {
                acc.push_str(s);
                Ok(acc)
            }
            Token::Number(n) => {
                if n.is_integer() && !n.is_neg() {
                    Ok(acc + &n.to_string())
                } else {
                    Err(Error::InvalidFunc(
                        "found decimal number in name".to_string(),
                    ))
                }
            }
            _ => unreachable!("Only captured String and Number"),
        })?;

    if func_name.is_empty() {
        return Err(Error::InvalidFunc("empty function name".to_string()));
    }

    let func_arg = items[1]
        .iter()
        .fold(String::new(), |mut acc, value| match value {
            Token::String(s) => {
                acc.push_str(s);
                acc
            }
            _ => unreachable!("Only captured String"),
        });

    let expr = parse(rhs)?;

    Ok(Func::new(func_name, func_arg, expr))
}
