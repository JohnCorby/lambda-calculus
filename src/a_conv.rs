use crate::ast::*;
use crate::intern::Intern;

impl Term {
    /// gives a unique name to every bound variable, no matter the scope
    pub fn a_conv(self) -> Self {
        self.a_conv_(&mut Default::default(), &mut 0)
    }

    fn a_conv_(self, scopes: &mut Vec<(Var, usize)>, id: &mut usize) -> Self {
        match self {
            Self::Var(var) => Self::Var(var.a_conv(scopes)),
            Self::Abs(abs) => Self::Abs(abs.a_conv(scopes, id)),
            Self::App(app) => Self::App(app.a_conv(scopes, id)),
        }
    }
}
impl Var {
    fn a_conv(mut self, scopes: &mut Vec<(Var, usize)>) -> Self {
        if let Some((_, id)) = scopes.iter().rfind(|(var, _)| *var == self) {
            self.0 = id.to_string().intern()
        }
        self
    }
}
impl Abs {
    fn a_conv(mut self, scopes: &mut Vec<(Var, usize)>, id: &mut usize) -> Self {
        scopes.push((self.param, *id));
        *id += 1;
        self.param = self.param.a_conv(scopes);
        *self.body = self.body.a_conv_(scopes, id);
        scopes.pop();
        self
    }
}
impl App {
    fn a_conv(mut self, scopes: &mut Vec<(Var, usize)>, id: &mut usize) -> Self {
        *self.left = self.left.a_conv_(scopes, id);
        *self.right = self.right.a_conv_(scopes, id);
        self
    }
}
