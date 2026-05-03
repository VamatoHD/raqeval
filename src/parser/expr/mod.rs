use crate::{Consts, Ctx, Func, Rational, lexer::Op};

mod reduce;
mod replace;

#[derive(Debug, Clone)]
pub enum Expr {
    Const(Consts),
    Number(Rational),
    Var(String),
    Infix {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Call {
        func: String,
        args: Vec<Expr>,
    },
    Log {
        base: Box<Expr>,
        arg: Box<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(c) => write!(f, "{}", c),
            Expr::Number(v) => write!(f, "{}", v),
            Expr::Var(v) => write!(f, "{}", v),
            Expr::Infix { lhs, op, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::Call { func, args } => {
                if args.len() == 0 {
                    write!(f, "{}()", func)
                } else {
                    let concat = args
                        .iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    write!(f, "{}({})", func, concat)
                }
            }
            Expr::Log { base, arg } => {
                //TODO: Ignore base if is equal to 10
                write!(f, "log({}, {})", base, arg)
            }
            _ => todo!(),
        }
    }
}

impl Expr {
    pub fn is_infinite(&self, ctx: &Ctx) -> bool {
        //BROKEN
        //TODO: Filter out duplicated functions
        self.into_iter()
            .filter_map(|expr| match expr {
                Expr::Call { func, .. } => ctx.get_func(func),
                _ => None,
            })
            .any(|func| func.is_recursive(ctx))
    }

    pub fn is_numeric(&self, ctx: Option<&Ctx>) -> bool {
        //BROKEN
        self.into_iter().all(|expr| match dbg!(expr) {
            Expr::Var(_) => false, //Isn't numeric if there is a variable
            Expr::Call { func, args } => match expr.get_inner_func(ctx) {
                Some(func) => func.get_expr().is_numeric(ctx),
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
            Expr::Call { args, .. } => {
                self.stack.extend(args);
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
