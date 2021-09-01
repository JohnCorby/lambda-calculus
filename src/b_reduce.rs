use crate::ast::*;

impl Term {
    pub fn b_reduce(mut self) -> Self {
        const ITERATIONS: usize = 100;
        for _ in 0..ITERATIONS {
            let last_self = self.clone();
            self = self.b_reduce_();
            if last_self == self {
                return self;
            }

            self = self.subst();
        }
        panic!("b_reduce didn't terminate after {} iterations", ITERATIONS);
    }

    fn b_reduce_(self) -> Self {
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
        *self.body = self.body.b_reduce_();
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
                *self.left = self.left.b_reduce_();
                *self.right = self.right.b_reduce_();
                Term::App(self)
            }
        }
    }
}
