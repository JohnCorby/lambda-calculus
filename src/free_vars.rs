use crate::ast::*;
use std::collections::HashSet;

impl Term {
    pub fn free_vars(&self) -> HashSet<Var> {
        match self {
            Self::Var(var) => var.free_vars(),
            Self::Abs(abs) => abs.free_vars(),
            Self::App(app) => app.free_vars(),
        }
    }
}
impl Var {
    fn free_vars(&self) -> HashSet<Var> {
        let mut set = HashSet::new();
        set.insert(*self);
        set
    }
}
impl Abs {
    fn free_vars(&self) -> HashSet<Var> {
        let mut set = self.body.free_vars();
        set.remove(&self.param);
        set
    }
}
impl App {
    fn free_vars(&self) -> HashSet<Var> {
        let mut set = self.func.free_vars();
        set.extend(self.arg.free_vars());
        set
    }
}
