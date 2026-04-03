use super::Expr;
use crate::{Ctx, Error, Func, lexer::Op};

impl Expr {
    pub fn reduce(&self, ctx: &Ctx) -> Result<Expr, Error> {
        Ok(match self {
            Expr::Infix { lhs, op, rhs } => {
                let lhs = lhs.reduce(ctx)?;
                let rhs = rhs.reduce(ctx)?;

                use Expr::Const;
                if let Const(a) = lhs
                    && let Const(b) = rhs
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

            Expr::Call { func, arg } => ctx
                .get_func(&func)
                .ok_or_else(|| Error::InvalidFunc(func.clone()))?
                .reduce(arg.reduce(ctx)?, ctx)?,

            v => v.clone(),
        })
    }
}
