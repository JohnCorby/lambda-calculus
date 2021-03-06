use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq)]
pub enum Term {
    Var(Var),
    Abs(Abs),
    App(App),
}
impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Var(var) => Display::fmt(var, f),
            Self::Abs(abs) => Display::fmt(abs, f),
            Self::App(app) => Display::fmt(app, f),
        }
    }
}
impl Debug for Term {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Var(pub &'static str);
impl Display for Var {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl Debug for Var {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Clone, PartialEq)]
pub struct Abs {
    pub param: Var,
    pub body: Box<Term>,
}
impl Display for Abs {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[{} -> {}]", self.param, self.body)
    }
}
impl Debug for Abs {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[derive(Clone, PartialEq)]
pub struct App {
    pub left: Box<Term>,
    pub right: Box<Term>,
}
impl Display for App {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({} {})", self.left, self.right)
    }
}
impl Debug for App {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
