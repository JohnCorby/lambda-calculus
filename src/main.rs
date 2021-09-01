//! https://en.wikipedia.org/wiki/Lambda_calculus

#![feature(option_result_unwrap_unchecked)]
#![feature(hash_set_entry)]
#![feature(box_patterns)]
#![feature(format_args_capture)]

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
    // input("(λx.λy.(λz.(λx.z x) (λy.z y)) (x y))").eval();
    // input(r"\entry book . Cons entry book").eval();

    let n0 = r"(\f x.x)";
    let n1 = r"(\f x.f x)";
    let n2 = r"(\f x.f (f x))";
    let n3 = r"(\f x.f (f (f x)))";

    let succ = "(λn.λf.λx.f (n f x))";
    let plus = format!("(λm.λn.m {succ} n)");
    let mult = format!("(λm.λn.m ({plus} n) {n0})");
    let pow = "(λb.λe.e b)";

    let n4 = format!("({succ} {n3})");
    let n5 = format!("({succ} {n4})");
    let n6 = format!("({succ} {n5})");
    let n7 = format!("({succ} {n6})");
    let n8 = format!("({succ} {n7})");

    let pred = "(λn.λf.λx.n (λg.λh.h (g f)) (λu.x) (λu.u))";
    let sub = format!("(λm.λn.n {pred} m)");

    let btrue = "(λx.λy.x)";
    let bfalse = "(λx.λy.y)";

    let and = r"(\p q.p q p)";
    let or = r"(\p q.p p q)";
    let not = format!(r"(\p.p {bfalse} {btrue})");
    let if_then_else = r"(\p a b.p a b)";

    let is_zero = format!(r"(\n.n (\x.{bfalse}) {btrue})");
    let leq = format!(r"(\m n.{is_zero} ({sub} m n))");

    let pair = r"(\x y f.f x y)";
    let first = format!(r"(\p.p {btrue})");
    let second = format!(r"(\p.p {bfalse})");
    let nil = format!(r"(\x.{btrue})");
    let is_null = format!(r"(\p.p (\x y.{bfalse}))");

    assert_eq!(
        input(format!("{succ} ({succ} {n0})")).eval(),
        input(n2).eval()
    );
    assert_eq!(input(format!("{plus} {n1} {n2}")).eval(), input(n3).eval());
    assert_eq!(input(format!("{mult} {n2} {n3}")).eval(), input(n6).eval());

    // assert_eq!(
    //     input(format!("{pow} {n1} {n3}")).eval(),
    //     input(n1).eval()
    // );

    assert_eq!(input(format!("{sub} {n3} {n1}")).eval(), input(n2).eval());

    assert_eq!(
        input(format!("{and} {btrue} {bfalse}")).eval(),
        input(bfalse).eval()
    );

    assert_eq!(input(format!("{is_zero} {n0}")).eval(), input(btrue).eval(),);

    assert_eq!(
        input(format!("{leq} {n1} {n2}")).eval(),
        input(btrue).eval(),
    );
    assert_eq!(
        input(format!("{leq} {n2} {n1}")).eval(),
        input(bfalse).eval(),
    );
    assert_eq!(
        input(format!("{leq} {n1} {n1}")).eval(),
        input(btrue).eval(),
    );

    let mapping = format!(r"(\x.{pair} ({second} x) ({succ} ({second} x)))");
    assert_eq!(
        input(format!("{mapping} ({pair} {n1} {n2})")).eval(),
        input(format!("{pair} {n2} {n3}")).eval()
    )
}

fn input(input: impl ToString) -> Term {
    Node::parse(input.to_string().intern(), Kind::input)
        .unwrap()
        .visit()
}
impl Term {
    fn eval(mut self) -> Self {
        println!("START {self}");
        // self = self.a_conv();
        // println!("a_conv {}", self);

        // loop {
        //     let last_self = self.clone();
        //     self = self.n_reduce();
        //     if last_self == self {
        //         break;
        //     }
        //     println!("n_reduce {}", self);
        // }

        self = self.b_reduce();
        // println!("b_reduce {}", self);

        // self = self.a_conv();
        println!("END {self}");
        self
    }
}
