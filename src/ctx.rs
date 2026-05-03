use crate::{Error, Func};
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
        let name = func.get_name().to_string();
        if self.vars.contains(&name) {
            Err(Error::OverlapElements(vec![name]))
        } else {
            self.funcs.insert(name, func);
            Ok(())
        }
    }

    pub fn get_func(&self, name: &str) -> Option<&Func> {
        self.funcs.get(name)
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

    pub fn var_exists(&self, var: &str) -> bool {
        self.vars.contains(var)
    }
}
