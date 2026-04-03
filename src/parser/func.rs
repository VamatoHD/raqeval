use crate::{Ctx, Expr};

pub const BUILTINS: [&str; 3] = ["sin", "cos", "tan"];

#[derive(Debug)]
pub enum Func {
    // name(arg) = expr
    Defined {
        name: String,
        arg: String,
        expr: Expr,
    },
    Builtin {
        name: String,
    },
}

impl Func {
    pub fn new(name: String, arg: String, expr: Expr) -> Func {
        if BUILTINS.contains(&name.as_str()) {
            Func::Builtin { name }
        } else {
            Func::Defined { name, arg, expr }
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Func::Defined { name, .. } => name.as_str(),
            Func::Builtin { name, .. } => name.as_str(),
        }
    }

    pub fn get_arg(&self) -> Option<&str> {
        match self {
            Func::Defined { arg, .. } => Some(arg.as_str()),
            Func::Builtin { .. } => None,
        }
    }

    pub fn get_expr(&self) -> Option<&Expr> {
        match self {
            Func::Defined { expr, .. } => Some(expr),
            Func::Builtin { .. } => None,
        }
    }

    pub fn is_recursive(&self, ctx: &Ctx) -> bool {
        use std::collections::HashSet;
        let mut visited: HashSet<String> = HashSet::new();
        let mut to_visit = match self.get_expr() {
            Some(expr) => vec![expr],
            None => return false, //Builtins aren't recursive
        };

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
                            match func.get_expr() {
                                Some(expr) => to_visit.push(expr),
                                _ => (),
                            };
                        }
                        to_visit.push(arg);
                        visited.insert(func.clone());
                    }
                }
                _ => (),
            }
        }

        false
    }
}
