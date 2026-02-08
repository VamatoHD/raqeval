use super::Expr;
use crate::{Ctx, Error, lexer::Op};

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
}
