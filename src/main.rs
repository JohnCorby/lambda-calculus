//! https://en.wikipedia.org/wiki/Lambda_calculus

#![feature(option_result_unwrap_unchecked)]
#![feature(hash_set_entry)]

use crate::a_convert::AConvert;
use crate::ast::Term;
use crate::parse::{Kind, Node};

mod a_convert;
mod ast;
mod intern;
mod parse;
mod visit;

fn main() {
    println!(
        "{}",
        Node::parse("M N P", Kind::input).unwrap().visit::<Term>()
    );
    println!(
        "{}",
        Node::parse("λx. M N", Kind::input).unwrap().visit::<Term>()
    );

    let node = Node::parse("λx. λy. x", Kind::input).unwrap();
    println!("{}", node);
    let term = node.visit::<Term>();
    println!("{}", term);
    println!("{}", term.a_convert(&mut Default::default()));

    let node = Node::parse("λx. λy. λz. x z (y z)", Kind::input).unwrap();
    println!("{}", node);
    let term = node.visit::<Term>();
    println!("{}", term);
    println!("{}", term.a_convert(&mut Default::default()));

    let node = Node::parse("λz. (λy. y (λx. x)) (λx. z x)", Kind::input).unwrap();
    println!("{}", node);
    let term = node.visit::<Term>();
    println!("{}", term);
    println!("{}", term.a_convert(&mut Default::default()));
}
