use crate::{Ctx, Expr};

#[derive(Debug)]
pub struct Func {
    pub(crate) name: String,
    pub(crate) arg: String,
    pub(crate) expr: Expr,
}

impl Func {
    pub fn new(name: String, arg: String, expr: Expr) -> Func {
        Func { name, arg, expr }
    }

    pub fn is_recursive(&self, ctx: &Ctx) -> bool {
        use std::collections::HashSet;
        let mut visited: HashSet<String> = HashSet::new();
        let mut to_visit = vec![&self.expr];

        while let Some(next) = to_visit.pop() {
            match next {
                Expr::Infix { lhs, op, rhs } => {
                    to_visit.push(lhs);
                    to_visit.push(rhs);
                }
                Expr::Call { func, arg } => {
                    if visited.contains(func) {
                        return true;
                    } else {
                        if let Some(func) = ctx.get_func(func) {
                            to_visit.push(&func.expr)
                        }
                        to_visit.push(arg);
                        visited.insert(func.clone());
                    }
                }
                _ => continue,
            }
        }

        false
    }
}
