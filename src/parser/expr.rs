use super::super::Ctx;
use super::{Error, Ident, Op, Rational};

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
    pub fn reduce(self, ctx: &Ctx) -> Result<Expr, Error> {
        Ok(match self {
            Expr::Infix { lhs, op, rhs } => {
                let lhs = lhs.reduce(ctx)?;
                let rhs = rhs.reduce(ctx)?;

                use Expr::Const;
                if let Const(a) = lhs
                    && let Const(b) = rhs
                {
                    let res = match op {
                        Op::Add => a + b,
                        Op::Sub => a - b,
                        Op::Mul => a * b,
                        Op::Div => a / b,
                        _ => unimplemented!(),
                    };

                    Const(res)
                } else {
                    Expr::Infix {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    }
                }
            }
            Expr::Call { func, arg } => {
                let func_expr = ctx
                    .get_func(&func)
                    .ok_or_else(|| Error::InvalidFunc(func))?;

                let expanded = func_expr
                    .replace_var(&"x".to_string(), arg.as_ref())
                    .unwrap_or_else(|| func_expr.clone());

                expanded.reduce(ctx)?
            }
            v => v,
        })
    }

    pub fn replace_var(&self, var: &String, new: &Expr) -> Option<Expr> {
        match self {
            Expr::Var(v) => {
                if v == var {
                    Some(new.clone())
                } else {
                    None
                }
            }
            Expr::Infix { lhs, op, rhs } => {
                let new_lhs = lhs.replace_var(var, new);
                let new_rhs = rhs.replace_var(var, new);

                if new_lhs.is_none() && new_rhs.is_none() {
                    None
                } else {
                    Some(Expr::Infix {
                        lhs: Box::new(new_lhs.unwrap_or_else(|| lhs.as_ref().clone())),
                        op: op.clone(),
                        rhs: Box::new(new_rhs.unwrap_or_else(|| rhs.as_ref().clone())),
                    })
                }
            }
            Expr::Call { func, arg } => Some(Expr::Call {
                func: func.to_string(),
                arg: Box::new(arg.replace_var(var, new)?),
            }),
            _ => None,
        }
    }
}
