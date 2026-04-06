use core::num;

use crate::{Ctx, Error, Expr, Rational, lexer::Op};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Builtin {
    Ln,
}

impl std::fmt::Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Builtin::Ln => write!(f, "ln"),
        }
    }
}

impl Builtin {
    pub const ALL: &[Builtin] = &[Builtin::Ln];

    pub fn from_str(str: &str) -> Option<Builtin> {
        match str.to_ascii_lowercase().as_str() {
            "ln" => Some(Builtin::Ln),
            _ => None,
        }
    }

    pub fn reduce(&self, call_arg: &Expr, ctx: &Ctx) -> Option<Expr> {
        match self {
            Builtin::Ln => match call_arg {
                // ln(a*b) = ln(a) + ln(b)
                // ln(a/b) = ln(a) - ln(b)
                Expr::Infix { lhs, op, rhs } if matches!(op, Op::Mul | Op::Div) => {
                    Some(Expr::Infix {
                        lhs: Box::new(Expr::Call {
                            func: "ln".to_string(),
                            arg: lhs.clone(),
                        }),
                        op: if matches!(op, Op::Mul) {
                            Op::Add
                        } else {
                            Op::Sub
                        },
                        rhs: Box::new(Expr::Call {
                            func: "ln".to_string(),
                            arg: rhs.clone(),
                        }),
                    })
                }

                //ln(1) = 0
                Expr::Const(number) if number == 1u8 => Some(Expr::Const(Rational::zero())),
                _ => None,
            },
            _ => None,
        }
    }
}
