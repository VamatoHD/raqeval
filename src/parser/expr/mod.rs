use crate::{Consts, Ctx, Rational, lexer::Op};

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
            Expr::Infix { lhs, op, rhs } => {
                use Expr::Number;
                match (op, lhs.as_ref(), rhs.as_ref()) {
                    (Op::Sub, Number(a), v) if a == 0u8 => write!(f, "(-{})", rhs),
                    _ => write!(f, "({} {} {})", lhs, op, rhs),
                }
            }
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
            Expr::Log { base, arg } => match base.as_ref() {
                Expr::Number(a) if a == 10u8 => {
                    write!(f, "log({})", arg)
                }
                Expr::Const(Consts::E) => {
                    write!(f, "ln({})", arg)
                }
                _ => write!(f, "log({}, {})", base, arg),
            },
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
        match self {
            Expr::Const(_) | Expr::Number(_) => true,
            // Every global in ctx is numeric
            Expr::Var(name) => ctx.is_some_and(|v| v.get_global(name).is_some()),
            Expr::Infix { lhs, rhs, .. } => lhs.is_numeric(ctx) && rhs.is_numeric(ctx),
            Expr::Log { base, arg } => base.is_numeric(ctx) && arg.is_numeric(ctx),
            //TODO: check function to see if has any global
            Expr::Call { func, args } => args.iter().all(|arg| arg.is_numeric(ctx)),
        }
    }
}

impl From<Rational> for Expr {
    #[inline(always)]
    fn from(value: Rational) -> Self {
        Expr::Number(value)
    }
}

impl From<&Rational> for Expr {
    #[inline(always)]
    fn from(value: &Rational) -> Self {
        Expr::Number(value.clone())
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
