use crate::{Ctx, Func, Rational, lexer::Op};

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
    pub fn is_infinite(&self, ctx: &Ctx) -> bool {
        //TODO: Filter out duplicated functions
        self.into_iter()
            .filter_map(|expr| match expr {
                Expr::Call { func, .. } => ctx.get_func(func),
                _ => None,
            })
            .any(|func| func.is_recursive(ctx))
    }

    pub fn is_numeric(&self, ctx: Option<&Ctx>) -> bool {
        self.into_iter().all(|expr| match dbg!(expr) {
            Expr::Var(_) => false, //Isn't numeric if there is a variable
            Expr::Call { func, .. } => match expr.get_inner_func(ctx) {
                Some(Func::Defined { expr, .. }) => expr.is_numeric(ctx),
                _ => true,
            },
            _ => true,
        })
    }

    #[inline]
    fn get_inner_func<'a>(&self, ctx: Option<&'a Ctx>) -> Option<&'a Func> {
        let Some(ctx) = ctx else { return None };
        match self {
            Expr::Call { func, .. } => ctx.get_func(func),
            _ => None,
        }
    }
}

pub struct ExprIter<'a> {
    stack: Vec<&'a Expr>,
}

impl<'a> Iterator for ExprIter<'a> {
    type Item = &'a Expr;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.stack.pop()?;

        match cur {
            Expr::Infix { lhs, rhs, .. } => {
                self.stack.push(rhs);
                self.stack.push(lhs);
            }
            Expr::Call { arg, .. } => {
                self.stack.push(arg);
            }
            _ => (),
        }

        Some(cur)
    }
}

impl<'a> IntoIterator for &'a Expr {
    type Item = &'a Expr;
    type IntoIter = ExprIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ExprIter { stack: vec![self] }
    }
}

impl<'a> ExprIter<'a> {
    pub fn push(&mut self, value: &'a Expr) {
        self.stack.push(value);
    }
}
