//! https://en.wikipedia.org/wiki/Lambda_calculus

#![feature(option_result_unwrap_unchecked)]
#![feature(hash_set_entry)]
#![feature(box_patterns)]

use crate::ast::Term;
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
    println!("{}", input(r"\x y z w.N"));
    println!("{}", input(r"\x.\y.\z.\w.N"));

    println!("{}", input("M N P"));
    println!("{}", input("M N P").a_conv());
    println!("{}", input("λx. M N x"));
    println!("{}", input("λx. M N x").a_conv());
    println!("{:?}", input("λx. M N x").free_vars());

    let term = input("λx. λy. x");
    println!("{}", term);
    println!("{}", term.a_conv());

    let term = input("λx. λy. λz. x z (y z)");
    println!("{}", term);
    println!("{}", term.a_conv());

    let term = input("λz. (λy. y (λx. x)) (λx. z x)");
    println!("{}", term);
    println!("{:?}", term.free_vars());
    println!("{}", term.a_conv());
    let term = input("λz. λx. λy. (x λx. x)");
    println!("{}", term);
    println!("{:?}", term.free_vars());
    println!("{}", term.a_conv());

    println!("{}", input("λx.x").a_eq(&input("λy.y")));
    println!("{}", input("λx.x").a_eq(&input("λx.y")));
    println!("{}", input("x").a_eq(&input("y")));

    let term = input("(λx.λy.(λz.(λx.z x) (λy.z y)) (x y))");
    println!("{}", term);
    println!("{:?}", term.free_vars());
    println!("{}", term.a_conv());

    println!("{}", input("(λV.M) N").b_reduce());
    println!(
        "{}",
        input("(λx.λy.(λz.(λx.z x) (λy.z y)) (x y))").b_reduce()
    );

    println!("{}", input(r"\x.f x").n_reduce());
    println!("{}", input(r"\x.(\z. x) x").n_reduce());

    // input(r"(\x.x x)(\x.x x)").run();
    input("(λx.λy.(λz.(λx.z x) (λy.z y)) (x y))").run();
    input(r"\entry book . Cons entry book").run();
}

fn input(input: &'static str) -> Term {
    Node::parse(input, Kind::input).unwrap().visit()
}
impl Term {
    fn run(mut self) -> Self {
        println!("START {}", self);
        // self = self.a_conv();
        // println!("a_conv {}", self);

        let mut last_self;
        loop {
            last_self = self.clone();
            self = self.n_reduce();
            if last_self == self {
                break;
            }
            println!("n_reduce {}", self);
        }

        const ITERATIONS: usize = 10;
        for _ in 0..ITERATIONS {
            last_self = self.clone();
            self = self.b_reduce();
            if last_self == self {
                println!("END {}", self);
                return self;
            }
            println!("b_reduce {}", self);

            self = self.subst();
            println!("subst {}", self);
        }
        panic!(
            "run {} didn't terminate after {} iterations",
            self, ITERATIONS
        )
    }
}
