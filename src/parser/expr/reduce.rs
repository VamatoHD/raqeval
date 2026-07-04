use super::Expr;
use crate::{Ctx, Error, Rational, lexer::Op};

impl Op {
    #[inline]
    pub fn apply(&self, a: &Rational, b: &Rational) -> Result<Expr, Error> {
        Ok(match self {
            Op::Add => (a + b).into(),
            Op::Sub => (a - b).into(),
            Op::Mul => (a * b).into(),
            Op::Div => (a / b).into(),
            Op::Exp => a.pow(b)?,
        })
    }
}

impl Expr {
    pub fn reduce(&self, ctx: &Ctx) -> Result<Expr, Error> {
        Ok(match self {
            Expr::Infix { lhs, op, rhs } => {
                let lhs = lhs.reduce(ctx)?;
                let rhs = rhs.reduce(ctx)?;

                use Expr::Number;

                if let (Number(a), Number(b)) = (&lhs, &rhs) {
                    return op.apply(a, b);
                }

                match (op, &lhs, &rhs) {
                    //Left size is zero
                    (Op::Add, Number(a), v) if a == 0u8 => return Ok(v.clone()),
                    (Op::Mul | Op::Div, Number(a), _) if a == 0u8 => {
                        return Ok(Number(Rational::zero()));
                    }

                    //Right side is zero
                    (Op::Add | Op::Sub, v, Number(a)) if a == 0u8 => return Ok(v.clone()),
                    (Op::Mul, _, Number(a)) if a == 0u8 => return Ok(Number(Rational::zero())),
                    (Op::Div, _, Number(a)) if a == 0u8 => return Err(Error::DivisionByZero),

                    // Mul and Div by one
                    (Op::Mul, Number(a), v) if a == 1u8 => return Ok(v.clone()),
                    (Op::Mul, v, Number(a)) if a == 1u8 => return Ok(v.clone()),
                    (Op::Div, v, Number(a)) if a == 1u8 => return Ok(v.clone()),

                    //Return default
                    (Op::Sub, Number(a), _) if a == 0u8 => {}
                    _ => {}
                }

                Expr::Infix {
                    lhs: Box::new(lhs),
                    op: op.clone(),
                    rhs: Box::new(rhs),
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

            // Logarithms
            Expr::Log { base, arg } => match arg.reduce(ctx)? {
                Expr::Infix { lhs, op, rhs } if matches!(op, Op::Mul | Op::Div) => {
                    // Both lhs and rhs should be reduced, as arg is reduced
                    let new_op = if op == Op::Mul { Op::Add } else { Op::Sub };
                    Expr::Infix {
                        lhs: Box::new(Expr::Log {
                            base: base.clone(),
                            arg: lhs.clone(),
                        }),
                        op: new_op,
                        rhs: Box::new(Expr::Log {
                            base: base.clone(),
                            arg: rhs.clone(),
                        }),
                    }
                    .reduce(ctx)?
                }
                Expr::Number(n) if n == 1u128 => Expr::Number(Rational::zero()),
                reduced => Expr::Log {
                    base: base.clone(),
                    arg: Box::new(reduced),
                },
            },

            // If was inside a function, it would have been replaced by now
            expr @ Expr::Var(name) => ctx
                .get_global(name)
                .cloned()
                .unwrap_or_else(|| expr.clone()),

            // No reduce options
            v => v.clone(),
        })
    }
}
