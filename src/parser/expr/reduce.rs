use super::Expr;
use crate::{Ctx, Error, Func, lexer::Op};

impl Expr {
    pub fn reduce(&self, ctx: &Ctx) -> Result<Expr, Error> {
        Ok(match self {
            Expr::Infix { lhs, op, rhs } => {
                let lhs = lhs.reduce(ctx)?;
                let rhs = rhs.reduce(ctx)?;

                use Expr::Const;
                if let Const(ref a) = lhs
                    && let Const(ref b) = rhs
                {
                    let res = op.apply(a, b);
                    Const(res)
                } else {
                    Expr::Infix {
                        lhs: Box::new(lhs),
                        op: op.clone(),
                        rhs: Box::new(rhs),
                    }
                }
            }

            Expr::Call { func, arg } => {
                let func_obj = ctx
                    .get_func(&func)
                    .ok_or_else(|| Error::InvalidFunc(func.clone()))?;

                let reduced_arg = arg.reduce(ctx)?;

                match func_obj {
                    Func::Builtin { inner } => {
                        let reduced = inner.reduce(&reduced_arg, ctx);

                        match reduced {
                            // Reduce the expression originated
                            Some(expr) => expr.reduce(ctx)?,
                            // Otherwise return itself
                            // func is a builtin, so no reducing
                            None => Expr::Call {
                                func: func.clone(),
                                arg: Box::new(reduced_arg),
                            },
                        }
                    }
                    Func::Defined { arg, expr, .. } => expr
                        .replace_var(arg, &reduced_arg)
                        .unwrap_or_else(|| expr.clone())
                        .reduce(ctx)?,
                }
            }

            v => v.clone(),
        })
    }
}
