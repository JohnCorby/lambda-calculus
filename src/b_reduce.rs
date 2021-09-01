use crate::ast::*;

impl Term {
    pub fn b_reduce(self) -> Self {
        match self {
            Self::Var(var) => Self::Var(var),
            Self::Abs(abs) => Self::Abs(abs.b_reduce()),
            Self::App(app) => app.b_reduce(),
            Self::Subst(subst) => unreachable!("b_reduce on subst {}", subst),
        }
    }
}
impl Abs {
    fn b_reduce(mut self) -> Self {
        *self.body = self.body.b_reduce();
        self
    }
}
impl App {
    fn b_reduce(mut self) -> Term {
        match *self.left {
            Term::Abs(abs) => Term::Subst(Subst {
                in_term: abs.body,
                from_var: abs.param,
                to_term: self.right,
            }),

            _ => {
                *self.left = self.left.b_reduce();
                *self.right = self.right.b_reduce();
                Term::App(self)
            }
        }
    }
}
