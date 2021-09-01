//! https://en.wikipedia.org/wiki/Lambda_calculus

#![feature(option_result_unwrap_unchecked)]
#![feature(hash_set_entry)]
#![feature(box_patterns)]

use crate::ast::Term;
use crate::intern::Intern;
use crate::parse::{Kind, Node};

mod a_conv;
mod ast;
mod b_reduce;
mod free_vars;
mod intern;
mod n_reduce;
mod parse;
mod subst;
mod visit;

fn main() {
    // input(r"(\x.x x)(\x.x x)").run();
    input("(λx.λy.(λz.(λx.z x) (λy.z y)) (x y))").eval();
    input(r"\entry book . Cons entry book").eval();

    const N0: &str = r"(\f x.x)";
    const N1: &str = r"(\f x.f x)";
    const N2: &str = r"(\f x.f (f x))";
    const N3: &str = r"(\f x.f (f (f x)))";

    const SUCC: &str = "(λn.λf.λx.f (n f x))";
    const PLUS: &str = "(λm.λn.λf.λx.m f (n f x))";

    assert_eq!(
        input(format!("{0} ({0} {1})", SUCC, N0).intern()).eval(),
        input(N2).eval()
    );

    assert_eq!(
        input(format!("{} {} {}", PLUS, N1, N2).intern()).eval(),
        input(N3).eval()
    );
}

fn input(input: &'static str) -> Term {
    Node::parse(input, Kind::input).unwrap().visit()
}
impl Term {
    fn eval(mut self) -> Self {
        println!("START {}", self);
        self = self.a_conv();
        println!("a_conv {}", self);

        // loop {
        //     last_self = self.clone();
        //     self = self.n_reduce();
        //     if last_self == self {
        //         break;
        //     }
        //     println!("n_reduce {}", self);
        // }

        self = self.b_reduce();
        println!("b_reduce {}", self);

        self = self.a_conv();
        println!("END {}", self);
        self
    }
}
