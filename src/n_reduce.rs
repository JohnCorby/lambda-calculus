use crate::ast::*;

impl Term {
    pub fn n_reduce(self) -> Self {
        match self {
            Self::Var(var) => Self::Var(var),
            Self::Abs(abs) => abs.n_reduce(),
            Self::App(app) => Self::App(app.n_reduce()),
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
            } if param == right_var && !param.is_free_in(&left) => *left,

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
