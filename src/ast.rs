use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Term {
    Var(Var),
    Abs(Abs),
    App(App),
}
impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Var(i) => Display::fmt(i, f),
            Self::Abs(i) => Display::fmt(i, f),
            Self::App(i) => Display::fmt(i, f),
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

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Abs {
    pub param: Var,
    pub body: Rc<Term>,
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

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct App {
    pub func: Rc<Term>,
    pub arg: Rc<Term>,
}
impl Display for App {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({} {})", self.func, self.arg)
    }
}
impl Debug for App {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
