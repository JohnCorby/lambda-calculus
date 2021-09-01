use crate::ast::*;

impl Term {
    pub fn subst(self) -> Self {
        match self {
            Self::Var(var) => Self::Var(var),
            Self::Abs(abs) => Self::Abs(abs.subst()),
            Self::App(app) => Self::App(app.subst()),
            Self::Subst(subst) => subst.subst(),
        }
    }
}
impl Abs {
    fn subst(mut self) -> Self {
        *self.body = self.body.subst();
        self
    }
}
impl App {
    fn subst(mut self) -> Self {
        *self.func = self.func.subst();
        *self.arg = self.arg.subst();
        self
    }
}
impl Subst {
    fn subst(self) -> Term {
        use Term::*;
        match *self.in_term {
            Var(in_var) if in_var != self.from_var => Var(in_var),
            Var(_) => *self.to_term,

            App(in_app) => App(self::App {
                func: Subst(self::Subst {
                    in_term: in_app.func,
                    from_var: self.from_var,
                    to_term: self.to_term.clone(),
                })
                .into(),
                arg: Subst(self::Subst {
                    in_term: in_app.arg,
                    from_var: self.from_var,
                    to_term: self.to_term,
                })
                .into(),
            }),

            Abs(in_abs)
                if self.from_var != in_abs.param
                    && !self.to_term.free_vars().contains(&in_abs.param) =>
            {
                Abs(self::Abs {
                    param: in_abs.param,
                    body: Subst(self::Subst {
                        in_term: in_abs.body,
                        from_var: self.from_var,
                        to_term: self.to_term,
                    })
                    .into(),
                })
            }
            Abs(in_abs) => Abs(in_abs),

            _ => Subst(self),
        }
    }
}
