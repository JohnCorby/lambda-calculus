//! turns pest ast into our own
//!
//! also calculates index for easy eq

use crate::ast::*;
use crate::parse::{Kind, Node};

trait Visit {
    fn visit(node: Node) -> Self;
}
impl Node {
    pub fn visit<V: Visit>(self) -> V {
        V::visit(self)
    }
}

impl Visit for Term {
    fn visit(node: Node) -> Self {
        match node.kind() {
            Kind::var => Self::Var(node.visit()),
            Kind::abs => Self::Abs(node.visit()),
            Kind::app => Self::App(node.visit()),
            _ => unreachable!(node),
        }
    }
}
impl Visit for Var {
    fn visit(node: Node) -> Self {
        Self(node.str())
    }
}
impl Visit for Abs {
    fn visit(node: Node) -> Self {
        let mut nodes = node.children();
        Self {
            param: nodes.next().unwrap().visit(),
            body: nodes.next().unwrap().visit::<Term>().into(),
        }
    }
}
impl Visit for App {
    fn visit(node: Node) -> Self {
        let mut nodes = node.children();
        // left assoc
        let mut app = Self {
            func: nodes.next().unwrap().visit::<Term>().into(),
            arg: nodes.next().unwrap().visit::<Term>().into(),
        };
        for node in nodes {
            app = Self {
                func: Term::App(app).into(),
                arg: node.visit::<Term>().into(),
            }
        }
        app
    }
}
