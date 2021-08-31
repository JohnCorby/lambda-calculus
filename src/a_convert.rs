use crate::ast::*;
use crate::intern::Intern;
use std::collections::HashMap;
use std::ops::Deref;

pub trait AConvert {
    fn a_convert(self, scope: &mut HashMap<Var, usize>) -> Self;
}

impl AConvert for Term {
    fn a_convert(self, scope: &mut HashMap<Var, usize>) -> Self {
        match self {
            Self::Var(i) => Self::Var(i.a_convert(scope)),
            Self::Abs(i) => Self::Abs(i.a_convert(scope)),
            Self::App(i) => Self::App(i.a_convert(scope)),
        }
    }
}
impl AConvert for Var {
    fn a_convert(self, scope: &mut HashMap<Var, usize>) -> Self {
        Self(scope.entry(self).or_default().to_string().intern())
    }
}
impl AConvert for Abs {
    fn a_convert(self, scope: &mut HashMap<Var, usize>) -> Self {
        self.param.a_convert(scope);
        for distance in scope.values_mut() {
            *distance += 1
        }
        let abs = Self {
            param: Var(""),
            body: self.body.deref().clone().a_convert(scope).into(),
        };
        scope.remove(&self.param);
        for distance in scope.values_mut() {
            *distance -= 1
        }
        abs
    }
}
impl AConvert for App {
    fn a_convert(self, scope: &mut HashMap<Var, usize>) -> Self {
        Self {
            func: self.func.deref().clone().a_convert(scope).into(),
            arg: self.arg.deref().clone().a_convert(scope).into(),
        }
    }
}
