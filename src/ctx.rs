use std::collections::{HashMap, HashSet};

use crate::{Expr, Func};

pub struct Ctx {
    funcs: HashMap<String, Func>,
    vars: HashSet<String>,
}

impl Ctx {
    pub fn new() -> Self {
        Ctx {
            funcs: HashMap::new(),
            vars: HashSet::new(),
        }
    }

    pub fn add_func(&mut self, func: Func) {
        self.funcs.insert(func.name.clone(), func);
    }

    pub fn get_func(&self, func: &String) -> Option<&Func> {
        self.funcs.get(func)
    }

    pub fn add_var(&mut self, var: &str) {
        self.vars.insert(var.to_string());
    }

    pub fn get_funcs_names(&self) -> Vec<&str> {
        self.funcs.iter().map(|(name, _)| name.as_str()).collect()
    }

    pub fn get_vars_names(&self) -> Vec<&str> {
        self.funcs
            .iter()
            .map(|(_, f)| f.arg.as_str())
            .chain(self.vars.iter().map(|x| x.as_str()))
            .collect()
    }
}
