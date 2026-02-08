use std::collections::HashMap;

use crate::Expr;

pub struct Ctx {
    funcs: HashMap<String, Expr>,
}

impl Ctx {
    pub fn new() -> Self {
        Ctx {
            funcs: HashMap::new(),
        }
    }

    pub fn add_func(&mut self, func: String, expr: Expr) {
        self.funcs.insert(func, expr);
    }

    pub fn get_func(&self, func: &String) -> Option<&Expr> {
        self.funcs.get(func)
    }
}
