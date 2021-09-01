use crate::ast::*;

impl Term {
    fn b_reduce(self) -> Self {
        self.a_conv().b_reduce_()
    }

    fn b_reduce_(self) -> Self {
        match self {
            Self::Var(var) => Self::Var(var),
            Self::Abs(abs) => Self::Abs(abs.b_reduce()),
            Self::App(app) => Self::App(app.b_reduce()),
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
    fn b_reduce(mut self) -> Self {
        *self.func = self.func.b_reduce_();
        *self.arg = self.arg.b_reduce_();
        self
    }
}
