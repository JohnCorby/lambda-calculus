use crate::ast::*;
use crate::intern::Intern;
use std::collections::HashMap;
use std::ops::Deref;

pub trait AConv {
    fn a_conv(self, bounded: &mut HashMap<Var, usize>) -> Self;

    fn a_eq(&self, other: &Self) -> bool
    where
        Self: Sized + Clone + PartialEq,
    {
        self.clone().a_conv(&mut Default::default())
            == other.clone().a_conv(&mut Default::default())
    }
}

impl AConv for Term {
    fn a_conv(self, bounded: &mut HashMap<Var, usize>) -> Self {
        match self {
            Self::Var(i) => Self::Var(i.a_conv(bounded)),
            Self::Abs(i) => Self::Abs(i.a_conv(bounded)),
            Self::App(i) => Self::App(i.a_conv(bounded)),
        }
    }
}
impl AConv for Var {
    fn a_conv(self, bounded: &mut HashMap<Var, usize>) -> Self {
        if bounded.contains_key(&self) {
            Self(format!("\\{}", bounded[&self]).intern())
        } else {
            self
        }
    }
}
impl AConv for Abs {
    fn a_conv(self, bounded: &mut HashMap<Var, usize>) -> Self {
        for distance in bounded.values_mut() {
            *distance += 1
        }
        bounded.insert(self.param, 1);
        let abs = Self {
            param: Var(""),
            body: self.body.deref().clone().a_conv(bounded).into(),
        };
        bounded.remove(&self.param);
        for distance in bounded.values_mut() {
            *distance -= 1
        }
        abs
    }
}
impl AConv for App {
    fn a_conv(self, scope: &mut HashMap<Var, usize>) -> Self {
        Self {
            func: self.func.deref().clone().a_conv(scope).into(),
            arg: self.arg.deref().clone().a_conv(scope).into(),
        }
    }
}
