use crate::{Expr, Func};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ctx {
    funcs: HashMap<String, Func>,
    globals: HashMap<String, Option<Expr>>,
}

impl Ctx {
    pub fn new() -> Self {
        Ctx {
            funcs: HashMap::new(),
            globals: HashMap::new(),
        }
    }

    pub fn add_func(&mut self, func: Func) -> () {
        self.funcs.insert(func.get_name().to_string(), func);
    }

    pub fn get_func(&self, name: &str) -> Option<&Func> {
        self.funcs.get(name)
    }

    pub fn get_funcs_names(&self) -> Vec<&str> {
        self.funcs.iter().map(|(name, _)| name.as_str()).collect()
    }

    pub fn add_global(&mut self, var: &str, expr: Option<Expr>) -> () {
        self.globals.insert(var.to_string(), expr);
    }

    pub fn get_global(&self, name: &str) -> Option<&Expr> {
        self.globals.get(name).and_then(|inner| inner.as_ref())
    }

    pub fn get_global_names(&self) -> Vec<&str> {
        self.globals.iter().map(|(name, _)| name.as_str()).collect()
    }
}
