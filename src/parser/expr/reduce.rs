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
                    // Ignore some cases:
                    // 0 - b = -b (No unary support as of now)
                    (Op::Sub, Number(a), _) if a == 0u8 => {}

                    // 0 + b = b + 0 = b
                    (Op::Add, Number(a), b) if a == 0u8 => return Ok(b.clone()),
                    (Op::Add, b, Number(a)) if a == 0u8 => return Ok(b.clone()),

                    // b - 0 = b
                    (Op::Sub, b, Number(a)) if a == 0u8 => return Ok(b.clone()),

                    // 0 * b = b * 0 = 0
                    (Op::Mul, Number(a), _) if a == 0u8 => return Ok(Number(Rational::zero())),
                    (Op::Mul, _, Number(a)) if a == 0u8 => return Ok(Number(Rational::zero())),

                    // b / 0 = Error
                    (Op::Div, _, Number(a)) if a == 0u8 => return Err(Error::DivisionByZero),

                    // 0 / b = 0, error if b = 0
                    //TODO: Check if denominator is zero
                    (Op::Div, Number(a), _) if a == 0u8 => {
                        return unimplemented!();
                    }

                    // 1 * b = b * 1 = b / 1 = b
                    (Op::Mul, Number(a), b) if a == 1u8 => return Ok(b.clone()),
                    (Op::Mul | Op::Div, b, Number(a)) if a == 1u8 => return Ok(b.clone()),

                    // b ^ 0 = 1,  1 ^ b = 1
                    (Op::Exp, b, Number(a)) if a == 0u8 => return Ok(Number(1u8.into())),
                    (Op::Exp, Number(a), b) if a == 1u8 => return Ok(Number(1u8.into())),

                    // b ^ 1 = b
                    (Op::Exp, b, Number(a)) if a == 1u8 => return Ok(b.clone()),

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
            Expr::Log { base, arg } => {
                let reduced_base = Box::new(base.reduce(ctx)?);
                match arg.reduce(ctx)? {
                    Expr::Infix { lhs: a, op, rhs: b } if matches!(op, Op::Mul | Op::Div) => {
                        // Both a (lhs) and b (rhs) should be reduced, as arg is reduced
                        // log(a*b) = log(a) + log(b)
                        // log(a/b) = log(a) - log(b)
                        let new_op = if op == Op::Mul { Op::Add } else { Op::Sub };
                        Expr::Infix {
                            lhs: Box::new(Expr::Log {
                                base: reduced_base.clone(),
                                arg: a.clone(),
                            }),
                            op: new_op,
                            rhs: Box::new(Expr::Log {
                                base: reduced_base,
                                arg: b.clone(),
                            }),
                        }
                        .reduce(ctx)?
                    }
                    Expr::Infix { lhs: a, op, rhs: b } if matches!(op, Op::Exp) => {
                        // Both a (lhs) and b (rhs) should be reduced, as arg is reduced
                        // log(a^b) = b * log(a)
                        // TODO: Actual expression is log(a^b) = b * log(|a|) if b is even
                        Expr::Infix {
                            lhs: b.clone(),
                            op: Op::Mul,
                            rhs: Box::new(Expr::Log {
                                base: reduced_base,
                                arg: a.clone(),
                            }),
                        }
                        .reduce(ctx)?
                    }
                    Expr::Number(n) if n == 1u128 => Expr::Number(Rational::zero()),
                    reduced => Expr::Log {
                        base: reduced_base,
                        arg: Box::new(reduced),
                    },
                }
            }

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
