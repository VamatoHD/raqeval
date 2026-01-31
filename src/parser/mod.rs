use crate::Error;
use crate::Rational;

mod lexer;
pub use lexer::{Assoc, Ident, Lexer, Op, Token};

mod expr;
pub use expr::Expr;

pub fn parse(input: &str) -> Result<Expr, Error> {
    let mut lexer = Lexer::new(input, None, None)?;
    compute_expr(&mut lexer, 1)
}

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
