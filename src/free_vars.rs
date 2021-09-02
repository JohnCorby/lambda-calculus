use crate::ast::*;
use std::collections::HashSet;

impl Term {
    fn free_vars(&self) -> HashSet<Var> {
        match self {
            Self::Var(var) => var.free_vars(),
            Self::Abs(abs) => abs.free_vars(),
            Self::App(app) => app.free_vars(),
        }
    }
}
impl Var {
    pub fn is_free_in(&self, term: &Term) -> bool {
        term.free_vars().contains(self)
    }

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
        let mut set = self.left.free_vars();
        set.extend(self.right.free_vars());
        set
    }
}
