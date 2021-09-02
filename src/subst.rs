use crate::ast::*;

impl Term {
    fn subst(self) -> Self {
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
        *self.left = self.left.subst();
        *self.right = self.right.subst();
        self
    }
}
impl Subst {
    pub fn subst(self) -> Term {
        use Term::*;
        match *self.in_term {
            Var(in_var) if in_var != self.from_var => Var(in_var),
            Var(_) => *self.to_term,

            App(in_app) => App(self::App {
                left: self::Subst {
                    in_term: in_app.left,
                    from_var: self.from_var,
                    to_term: self.to_term.clone(),
                }
                .subst()
                .into(),
                right: self::Subst {
                    in_term: in_app.right,
                    from_var: self.from_var,
                    to_term: self.to_term,
                }
                .subst()
                .into(),
            }),

            Abs(in_abs)
                if self.from_var != in_abs.param && !in_abs.param.is_free_in(&self.to_term) =>
            {
                Abs(self::Abs {
                    param: in_abs.param,
                    body: self::Subst {
                        in_term: in_abs.body,
                        from_var: self.from_var,
                        to_term: self.to_term,
                    }
                    .subst()
                    .into(),
                })
            }
            Abs(in_abs) => Abs(in_abs),

            _ => Subst(self),
        }
    }
}
