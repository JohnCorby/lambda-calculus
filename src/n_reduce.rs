use crate::ast::*;

impl Term {
    pub fn n_reduce(self, changed: &mut bool) -> Self {
        match self {
            Self::Var(var) => Self::Var(var),
            Self::Abs(abs) => abs.n_reduce(changed),
            Self::App(app) => Self::App(app.n_reduce(changed)),
        }
    }
}
impl Abs {
    fn n_reduce(mut self, changed: &mut bool) -> Term {
        match self {
            Abs {
                param,
                body:
                    box Term::App(App {
                        left,
                        right: box Term::Var(right_var),
                    }),
            } if param == right_var && !param.is_free_in(&left) => {
                *changed = true;
                *left
            }

            _ => {
                *self.body = self.body.n_reduce(changed);
                Term::Abs(self)
            }
        }
    }
}
impl App {
    fn n_reduce(mut self, changed: &mut bool) -> Self {
        *self.left = self.left.n_reduce(changed);
        *self.right = self.right.n_reduce(changed);
        self
    }
}
