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
            Expr::Call { func, args } => {
                let mut new_args: Option<Vec<Expr>> = None;

                for (i, arg) in args.iter().enumerate() {
                    if let Some(replaced_arg) = arg.replace_var(var, new) {
                        // Lazily initialize the new vector
                        let vec = new_args.get_or_insert_with(|| {
                            let mut v = Vec::with_capacity(args.len());
                            // Copy all unchanged arguments
                            v.extend(args[..i].iter().cloned());
                            v
                        });
                        vec.push(replaced_arg);
                    } else if let Some(vec) = &mut new_args {
                        // If the vector was already created, push a copy
                        vec.push(arg.clone());
                    }
                }

                new_args.map(|args| Expr::Call {
                    func: func.clone(),
                    args,
                })
            }
            _ => None,
        }
    }
}
