use super::Expr;
use crate::{Ctx, Error};

impl Expr {
    pub fn reduce(&self, ctx: &Ctx) -> Result<Expr, Error> {
        Ok(match self {
            Expr::Infix { lhs, op, rhs } => {
                let lhs = lhs.reduce(ctx)?;
                let rhs = rhs.reduce(ctx)?;

                use Expr::Number;
                if let Number(ref a) = lhs
                    && let Number(ref b) = rhs
                {
                    let res = op.apply(a, b);
                    Number(res)
                } else {
                    Expr::Infix {
                        lhs: Box::new(lhs),
                        op: op.clone(),
                        rhs: Box::new(rhs),
                    }
                }
            }

            Expr::Call { func, args } => {
                let func_obj = ctx
                    .get_func(&func)
                    .ok_or_else(|| Error::InvalidFunc(func.clone()))?;

                let reduced_args = args
                    .iter()
                    .map(|arg| arg.reduce(ctx))
                    .collect::<Result<Vec<Expr>, Error>>()?;

                func_obj.apply_args(&reduced_args).reduce(ctx)?
            }

            v => v.clone(),
        })
    }
}
