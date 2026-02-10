use crate::Expr;

#[derive(Debug)]
pub struct Func {
    pub(crate) name: String,
    pub(crate) arg: String,
    pub(crate) expr: Expr,
}

impl Func {
    pub fn new(name: &str, arg: &str, expr: Expr) -> Func {
        Func {
            name: name.to_owned(),
            arg: arg.to_owned(),
            expr,
        }
    }
}
