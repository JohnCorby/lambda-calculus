use crate::ast::*;
use std::collections::HashSet;

impl Var {
    pub fn free_vars(&self) -> HashSet<Var> {
        let mut set = HashSet::new();
        set.insert(*self);
        set
    }
}
impl Abs {
    pub fn free_vars(&self) -> HashSet<Var> {
        let mut set = self.body.free_vars();
        set.remove(&self.param);
        set
    }
}
impl App {
    pub fn free_vars(&self) -> HashSet<Var> {
        let mut set = self.func.free_vars();
        set.extend(self.arg.free_vars());
        set
    }
}
impl Term {
    pub fn free_vars(&self) -> HashSet<Var> {
        match self {
            Self::Var(v) => v.free_vars(),
            Self::Abs(a) => a.free_vars(),
            Self::App(a) => a.free_vars(),
        }
    }
}

// pub struct Substitution {
//     in_what: Term,
//     from_what: Var,
//     to_what: Term,
// }
