use crate::Error;
use crate::Rational;

mod lexer;
pub use lexer::{Assoc, Ident, Lexer, Op, Token};

#[derive(Debug)]
pub enum Expr {
    Const(Rational),
    Ident(Ident),
    Prefix {
        op: Op,
        rhs: Box<Expr>,
    },
    Infix {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(v) => write!(f, "{}", v),
            Expr::Ident(i) => write!(f, "{}", i),
            Expr::Infix { lhs, op, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
            Expr::Prefix { op, rhs } => write!(f, "{}{}", op, rhs),
        }
    }
}

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
