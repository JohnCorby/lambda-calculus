//! https://en.wikipedia.org/wiki/Lambda_calculus

#![feature(option_result_unwrap_unchecked)]
#![feature(hash_set_entry)]

use crate::a_conv::AConv;
use crate::ast::Term;
use crate::parse::{Kind, Node};

mod a_conv;
mod ast;
mod intern;
mod parse;
mod visit;

fn main() {
    println!("{}", input("M N P"));
    println!("{}", input("M N P").a_conv(&mut Default::default()));
    println!("{}", input("λx. M N x"));
    println!("{}", input("λx. M N x").a_conv(&mut Default::default()));

    let term = input("λx. λy. x");
    println!("{}", term);
    println!("{}", term.a_conv(&mut Default::default()));

    let term = input("λx. λy. λz. x z (y z)");
    println!("{}", term);
    println!("{}", term.a_conv(&mut Default::default()));

    let term = input("λz. (λy. y (λx. x)) (λx. z x)");
    println!("{}", term);
    println!("{}", term.a_conv(&mut Default::default()));

    println!("{}", input("λx.x").a_eq(input("λy.y")));
    println!("{}", input("λx.x").a_eq(input("λx.y")));
    println!("{}", input("x").a_eq(input("y")));
}

fn input(input: &'static str) -> Term {
    Node::parse(input, Kind::input).unwrap().visit()
}
