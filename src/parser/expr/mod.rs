use crate::{Rational, lexer::Op};

mod reduce;
mod replace;

#[derive(Debug, Clone)]
pub enum Expr {
    Const(Rational),
    Var(String),
    Infix {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Call {
        func: String,
        arg: Box<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(v) => write!(f, "{}", v),
            Expr::Var(v) => write!(f, "{}", v),
            Expr::Infix { lhs, op, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::Call { func, arg } => write!(f, "{}({})", func, arg),
        }
    }
}
