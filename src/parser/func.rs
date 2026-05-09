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
        fn dfs(expr: &Expr, ctx: &Ctx, call_stack: &mut Vec<String>) -> bool {
            match expr {
                Expr::Const(_) | Expr::Number(_) | Expr::Var(_) => false,
                Expr::Infix { lhs, rhs, .. } => {
                    dfs(lhs, ctx, call_stack) || dfs(rhs, ctx, call_stack)
                }
                Expr::Log { base, arg } => dfs(base, ctx, call_stack) || dfs(arg, ctx, call_stack),
                Expr::Call { func, args } => {
                    if call_stack.contains(func) {
                        return true;
                    }

                    call_stack.push(func.clone());
                    let has_cycle = if let Some(f) = ctx.get_func(func) {
                        dfs(f.get_expr(), ctx, call_stack)
                    } else {
                        false
                    };

                    call_stack.pop();

                    if has_cycle {
                        return true;
                    }

                    args.iter().any(|arg| dfs(arg, ctx, call_stack))
                }
            }
        }

        let mut stack = vec![self.get_name().to_string()];
        dfs(self.get_expr(), ctx, &mut stack)
    }
}

