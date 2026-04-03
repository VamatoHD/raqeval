use crate::{Error, Expr, ctx, lexer::Op, parser::func};

use super::*;

impl Func {
    pub fn reduce(&self, call_arg: Expr, ctx: &Ctx) -> Result<Expr, Error> {
        dbg!(&self);
        match self {
            Func::Defined { name, arg, expr } => expr
                .replace_var(arg, &call_arg)
                .unwrap_or_else(|| expr.clone())
                .reduce(ctx),

            Func::Builtin { name } => match name.as_str() {
                "ln" => match &call_arg {
                    Expr::Infix { lhs, op, rhs } if matches!(op, Op::Mul) => Some(Expr::Infix {
                        lhs: Box::new(Expr::Call {
                            func: "ln".to_string(),
                            arg: lhs.clone(),
                        }),
                        op: Op::Add,
                        rhs: Box::new(Expr::Call {
                            func: "ln".to_string(),
                            arg: rhs.clone(),
                        }),
                    }),
                    Expr::Infix { lhs, op, rhs } if matches!(op, Op::Div) => Some(Expr::Infix {
                        lhs: Box::new(Expr::Call {
                            func: "ln".to_string(),
                            arg: lhs.clone(),
                        }),
                        op: Op::Sub,
                        rhs: Box::new(Expr::Call {
                            func: "ln".to_string(),
                            arg: rhs.clone(),
                        }),
                    }),
                    _ => None,
                },

                _ => None,
            }
            // If not updated, map to a expr::call
            .map_or_else(
                || {
                    Ok(Expr::Call {
                        func: name.clone(),
                        arg: Box::new(call_arg),
                    })
                },
                |v| Ok(v),
            ),
        }
    }
}
