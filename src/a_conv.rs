use crate::ast::*;
use crate::intern::Intern;
use std::collections::HashMap;

impl Term {
    /// also applies unique name for every bound variable, no matter the scope,
    /// rather than just fixing shadowing
    pub fn a_conv(self) -> Self {
        self.a_conv_(&mut HashMap::new(), &mut 0)
    }
    pub fn a_eq(&self, other: &Self) -> bool {
        self.clone().a_conv() == other.clone().a_conv_(&mut HashMap::new(), &mut 0)
    }

    fn a_conv_(self, bounded: &mut HashMap<Var, usize>, next: &mut usize) -> Self {
        match self {
            Self::Var(var) => Self::Var(var.a_conv(bounded)),
            Self::Abs(abs) => Self::Abs(abs.a_conv(bounded, next)),
            Self::App(app) => Self::App(app.a_conv(bounded, next)),
            Self::Subst(subst) => unreachable!("a_conv on subst {}", subst),
        }
    }
}
impl Var {
    fn a_conv(mut self, bounded: &mut HashMap<Var, usize>) -> Self {
        if bounded.contains_key(&self) {
            self.0 = format!("\\{}", bounded[&self]).intern()
        }
        self
    }
}
impl Abs {
    fn a_conv(mut self, bounded: &mut HashMap<Var, usize>, next: &mut usize) -> Self {
        bounded.insert(self.param, *next);
        *next += 1;
        self.param = self.param.a_conv(bounded);
        *self.body = self.body.a_conv_(bounded, next);
        bounded.remove(&self.param);
        self
    }
}
impl App {
    fn a_conv(mut self, bounded: &mut HashMap<Var, usize>, next: &mut usize) -> Self {
        *self.left = self.left.a_conv_(bounded, next);
        *self.right = self.right.a_conv_(bounded, next);
        self
    }
}
