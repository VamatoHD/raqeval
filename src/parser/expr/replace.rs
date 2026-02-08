use super::Expr;

impl Expr {
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
