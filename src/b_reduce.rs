use crate::ast::*;

impl Term {
    pub fn b_reduce(self) -> Self {
        match self {
            Self::Var(var) => Self::Var(var),
            Self::Abs(abs) => Self::Abs(abs.b_reduce()),
            Self::App(app) => app.b_reduce(),
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
            Term::Abs(abs) => abs.body.subst(abs.param, *self.right),

            _ => {
                *self.left = self.left.b_reduce();
                *self.right = self.right.b_reduce();
                Term::App(self)
            }
        }
    }
}

impl Term {
    fn subst(self, var: Var, term: Self) -> Self {
        match self {
            Term::Var(this) if this != var => Term::Var(this),
            Term::Var(_) => term,

            Term::App(this) => Term::App(App {
                left: this.left.subst(var, term.clone()).into(),
                right: this.right.subst(var, term).into(),
            }),

            Term::Abs(this) if var != this.param && !this.param.is_free_in(&term) => {
                Term::Abs(Abs {
                    param: this.param,
                    body: this.body.subst(var, term).into(),
                })
            }
            Term::Abs(this) => Term::Abs(this),
        }
    }
}
