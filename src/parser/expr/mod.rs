use crate::{Ctx, Rational, lexer::Op};

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

impl Expr {
    pub fn find_all<F>(&self, pred: F) -> Vec<&Expr>
    where
        F: Fn(&Expr) -> bool,
    {
        let mut queue = vec![self];
        let mut result = vec![];

        while let Some(next) = queue.pop() {
            if pred(next) {
                result.push(next);
            }
            match next {
                Expr::Infix { lhs, op: _, rhs } => {
                    queue.push(lhs);
                    queue.push(rhs);
                }
                Expr::Call { func: _, arg } => {
                    queue.push(arg);
                }
                _ => continue,
            }
        }

        result
    }

    pub fn is_infinite(&self, ctx: &Ctx) -> bool {
        //TODO: Filter out duplicated functions

        self.find_all(|expr| matches!(expr, Expr::Call { func: _, arg: _ }))
            .iter()
            .filter_map(|expr| match expr {
                Expr::Call { func, arg: _ } => ctx.get_func(func),
                _ => None, //unreachable!()
            })
            .any(|func| func.is_recursive(ctx))
    }
}
