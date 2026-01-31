use super::{Ident, Op, Rational};

#[derive(Debug)]
pub enum Expr {
    Const(Rational),
    Ident(Ident),
    Prefix {
        op: Op,
        rhs: Box<Expr>,
    },
    Infix {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(v) => write!(f, "{}", v),
            Expr::Ident(i) => write!(f, "{}", i),
            Expr::Infix { lhs, op, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
            Expr::Prefix { op, rhs } => write!(f, "{}{}", op, rhs),
        }
    }
}

impl Expr {
    pub fn reduce(self) -> Expr {
        match self {
            Expr::Infix { lhs, op, rhs } => {
                let lhs = lhs.reduce();
                let rhs = rhs.reduce();

                use Expr::Const;
                if let Const(a) = lhs
                    && let Const(b) = rhs
                {
                    let res = match op {
                        Op::Add => a + b,
                        Op::Sub => a + (-b),
                        Op::Mul => a * b,
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
            v => v,
        }
    }
}
