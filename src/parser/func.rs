use crate::{Ctx, Expr};

#[derive(Debug)]
pub struct Func {
    // name(arg) = expr
    name: String,
    args: Vec<String>,
    expr: Expr,
}

impl Func {
    pub fn new(name: String, args: Vec<String>, expr: Expr) -> Func {
        Func { name, args, expr }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn get_expr(&self) -> &Expr {
        &self.expr
    }

    pub fn apply_args(&self, values: &Vec<Expr>) -> Expr {
        let mut new_expr = self.expr.clone();
        for (i, func_arg) in self.args.iter().enumerate() {
            //TODO: Add error handling on fewer arguments passed
            let value = values.get(i).unwrap();
            if let Some(expr) = new_expr.replace_var(func_arg, value) {
                new_expr = expr;
            }
        }
        return new_expr;
    }

    pub fn is_recursive(&self, ctx: &Ctx) -> bool {
        //BROKEN
        use std::collections::HashSet;
        let mut visited: HashSet<String> = HashSet::new();
        let mut to_visit = vec![self.get_expr()];

        while let Some(next) = to_visit.pop() {
            match next {
                Expr::Infix { lhs, op, rhs } => {
                    to_visit.push(lhs);
                    to_visit.push(rhs);
                }
                Expr::Call { func, args } => {
                    if visited.contains(func) {
                        return true;
                    } else {
                        if let Some(func) = ctx.get_func(func) {
                            to_visit.push(func.get_expr())
                        }
                        to_visit.extend(args);
                        visited.insert(func.clone());
                    }
                }
                _ => (),
            }
        }

        false
    }
}
