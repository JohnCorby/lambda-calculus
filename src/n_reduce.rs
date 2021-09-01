use crate::ast::*;

impl Term {
    /// yeah i dont really like this one
    ///
    /// so i dont use it
    pub fn n_reduce(self) -> Self {
        match self {
            Self::Var(var) => Self::Var(var),
            Self::Abs(abs) => abs.n_reduce(),
            Self::App(app) => Self::App(app.n_reduce()),
            Self::Subst(subst) => unreachable!("n_reduce on subst {}", subst),
        }
    }
}
impl Abs {
    fn n_reduce(mut self) -> Term {
        match self {
            Abs {
                param,
                body:
                    box Term::App(App {
                        left,
                        right: box Term::Var(right_var),
                    }),
            } if param == right_var && !left.free_vars().contains(&right_var) => *left,

            _ => {
                *self.body = self.body.n_reduce();
                Term::Abs(self)
            }
        }
    }
}
impl App {
    fn n_reduce(mut self) -> Self {
        *self.left = self.left.n_reduce();
        *self.right = self.right.n_reduce();
        self
    }
}
