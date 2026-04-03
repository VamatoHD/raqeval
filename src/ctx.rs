use crate::{BUILTINS, Error, Expr, Func};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static BUILTIN_FUNCS: LazyLock<HashMap<String, Func>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for name in BUILTINS {
        map.insert(
            name.to_string(),
            Func::Builtin {
                name: name.to_string(),
            },
        );
    }
    map
});

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
        match &func {
            Func::Builtin { name, .. } => Err(Error::AssignBuiltinFunc(name.clone())),
            Func::Defined { name, expr, .. } => {
                if self.vars.contains(name) {
                    Err(Error::OverlapElements(vec![name.clone()]))
                } else {
                    self.funcs.insert(name.clone(), func);
                    Ok(())
                }
            }
        }
    }

    pub fn get_func(&self, name: &String) -> Option<&Func> {
        BUILTIN_FUNCS.get(name).or_else(|| self.funcs.get(name))
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
            .filter_map(|(_, f)| f.get_arg())
            .chain(self.vars.iter().map(|x| x.as_str()))
            .collect()
    }

    pub fn var_exists(&self, var: &str) -> bool {
        self.vars.contains(var)
    }
}
