use crate::{Error, Expr, Func};
use std::collections::{HashMap, HashSet};

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

    pub fn add_func(&mut self, func: Func) -> Result<(), Error> {
        if self.vars.contains(&func.name) {
            Err(Error::OverlapElements(vec![func.name.clone()]))
        } else {
            self.funcs.insert(func.name.clone(), func);
            Ok(())
        }
    }

    pub fn get_func(&self, func: &String) -> Option<&Func> {
        self.funcs.get(func)
    }

    pub fn get_funcs_names(&self) -> Vec<&str> {
        self.funcs.iter().map(|(name, _)| name.as_str()).collect()
    }

    pub fn add_var(&mut self, var: &str) -> Result<(), Error> {
        if self.funcs.contains_key(var) {
            Err(Error::OverlapElements(vec![var.to_string()]))
        } else {
            self.vars.insert(var.to_string());
            Ok(())
        }
    }

    pub fn get_vars_names(&self) -> Vec<&str> {
        self.funcs
            .iter()
            .map(|(_, f)| f.arg.as_str())
            .chain(self.vars.iter().map(|x| x.as_str()))
            .collect()
    }

    pub fn var_exists(&self, var: &str) -> bool {
        self.vars.contains(var)
    }
}
