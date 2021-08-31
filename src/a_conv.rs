use crate::ast::*;
use crate::intern::Intern;
use std::collections::HashMap;
use std::ops::Deref;

pub trait AConv {
    fn a_conv(self, bounded: &mut HashMap<Var, usize>, next: &mut usize) -> Self;

    fn a_eq(&self, other: &Self) -> bool
    where
        Self: Sized + Clone + PartialEq,
    {
        self.clone().a_conv(&mut HashMap::new(), &mut 0)
            == other.clone().a_conv(&mut HashMap::new(), &mut 0)
    }
}

impl AConv for Term {
    fn a_conv(self, bounded: &mut HashMap<Var, usize>, next: &mut usize) -> Self {
        match self {
            Self::Var(i) => Self::Var(i.a_conv(bounded, next)),
            Self::Abs(i) => Self::Abs(i.a_conv(bounded, next)),
            Self::App(i) => Self::App(i.a_conv(bounded, next)),
        }
    }
}
impl AConv for Var {
    fn a_conv(mut self, bounded: &mut HashMap<Var, usize>, _next: &mut usize) -> Self {
        if bounded.contains_key(&self) {
            self.0 = format!("\\{}", bounded[&self]).intern()
        }
        self
    }
}
impl AConv for Abs {
    fn a_conv(mut self, bounded: &mut HashMap<Var, usize>, next: &mut usize) -> Self {
        bounded.insert(self.param, *next);
        self.param = self.param.a_conv(bounded, next);
        *next += 1;
        self.body = self.body.deref().clone().a_conv(bounded, next).into();
        bounded.remove(&self.param);
        self
    }
}
impl AConv for App {
    fn a_conv(mut self, bounded: &mut HashMap<Var, usize>, next: &mut usize) -> Self {
        self.func = self.func.deref().clone().a_conv(bounded, next).into();
        self.arg = self.arg.deref().clone().a_conv(bounded, next).into();
        self
    }
}
