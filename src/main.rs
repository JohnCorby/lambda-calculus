//! https://en.wikipedia.org/wiki/Lambda_calculus

#![feature(option_result_unwrap_unchecked)]
#![feature(hash_set_entry)]
#![feature(box_patterns)]
#![feature(format_args_capture)]

mod a_conv;
mod ast;
mod b_reduce;
mod free_vars;
mod intern;
mod n_reduce;
mod parse;
mod visit;

fn main() {
    tests::factorial();
    // panics
    tests::infinite_recursion();
}

#[allow(dead_code)]
mod tests {
    use crate::ast::Term;
    use crate::parse::{Kind, Node};
    use const_format::*;

    fn test(left: &'static str, right: &'static str) {
        fn eval(input: &'static str) -> Term {
            println!("- input -    {input}");
            let node = Node::parse(input, Kind::input).unwrap();
            println!("- node -     {node}");
            let mut term = node.visit::<Term>();
            println!("- term -     {term}");
            term = term.a_conv();
            println!("- a_conv -   {term}");

            const ITERATIONS: usize = 1000;
            let mut terminated = false;
            for _ in 0..ITERATIONS {
                let mut changed = false;
                term = term.b_reduce(&mut changed);
                if !changed {
                    terminated = true;
                    break;
                }
                println!("- b_reduce - {term}");
            }
            if !terminated {
                panic!("b_reduce didn't terminate after {} iterations", ITERATIONS)
            }

            loop {
                let mut changed = false;
                term = term.n_reduce(&mut changed);
                if !changed {
                    break;
                }
                println!("- n_reduce - {term}");
            }

            term = term.a_conv();
            println!("- a_conv -   {term}");
            term
        }

        println!("--- TEST ---");
        println!("--- left ---");
        let term1 = eval(left);
        println!("--- right ---");
        let term2 = eval(right);
        assert_eq!(term1, term2);
        println!();
    }

    const _0: &str = r"(\f,x.x)";
    const _1: &str = r"(\f,x.f x)";
    const _2: &str = r"(\f,x.f (f x))";
    const _3: &str = r"(\f,x.f (f (f x)))";

    const SUCC: &str = "(λn.λf.λx.f (n f x))";

    const _4: &str = formatcp!("({SUCC} {_3})");
    const _5: &str = formatcp!("({SUCC} {_4})");
    const _6: &str = formatcp!("({SUCC} {_5})");
    const _7: &str = formatcp!("({SUCC} {_6})");
    const _8: &str = formatcp!("({SUCC} {_7})");
    #[test]
    fn succ() {
        test(formatcp!("{SUCC} {_0}"), _1);
        test(formatcp!("{SUCC} ({SUCC} {_0})"), _2);
    }
    const ADD: &str = formatcp!("(λm.λn.m {SUCC} n)");
    #[test]
    fn add() {
        test(formatcp!("{ADD} {_1} {_2}"), _3);
        test(formatcp!("{ADD} {_2} {_2}"), _4);
    }
    const MUL: &str = formatcp!("(λm.λn.m ({ADD} n) {_0})");
    #[test]
    fn mul() {
        test(formatcp!("{MUL} {_2} {_3}"), _6);
        test(formatcp!("{MUL} {_4} {_2}"), _8);
    }
    const POW: &str = "(λb.λe.e b)";
    #[test]
    fn pow() {
        test(formatcp!("{POW} {_2} {_3}"), _8);
        test(formatcp!("{POW} {_1} {_3}"), _1);
    }

    const PRED: &str = "(λn.λf.λx.n (λg.λh.h (g f)) (λu.x) (λu.u))";
    const SUB: &str = formatcp!("(λm.λn.n {PRED} m)");
    #[test]
    fn sub() {
        test(formatcp!("{SUB} {_3} {_1}"), _2);
        test(formatcp!("{SUB} {_2} {_5}"), _0);
    }

    const TRUE: &str = "(λx.λy.x)";
    const FALSE: &str = "(λx.λy.y)";

    const AND: &str = r"(\p,q.p q p)";
    const OR: &str = r"(\p,q.p p q)";
    const NOT: &str = formatcp!(r"(\p.p {FALSE} {TRUE})");
    const IF_THEN_ELSE: &str = r"(\p,a,b.p a b)";

    #[test]
    fn logic() {
        test(formatcp!("{AND} {FALSE} {FALSE}"), FALSE);
        test(formatcp!("{AND} {TRUE} {FALSE}"), FALSE);
        test(formatcp!("{AND} {FALSE} {TRUE}"), FALSE);
        test(formatcp!("{AND} {TRUE} {TRUE}"), TRUE);

        test(formatcp!("{OR} {FALSE} {FALSE}"), FALSE);
        test(formatcp!("{OR} {TRUE} {FALSE}"), TRUE);
        test(formatcp!("{OR} {FALSE} {TRUE}"), TRUE);
        test(formatcp!("{OR} {TRUE} {TRUE}"), TRUE);

        test(formatcp!("{NOT} {FALSE}"), TRUE);
        test(formatcp!("{NOT} {TRUE}"), FALSE);

        test(formatcp!("{IF_THEN_ELSE} {TRUE} {_2} {_3}"), _2);
        test(formatcp!("{IF_THEN_ELSE} {FALSE} {_2} {_3}"), _3);
    }
    const IS_ZERO: &str = formatcp!(r"(\n.n (\x.{FALSE}) {TRUE})");
    #[test]
    fn is_zero() {
        test(formatcp!("{IS_ZERO} {_0}"), TRUE);
        test(formatcp!("{IS_ZERO} {_3}"), FALSE);
    }
    const LEQ: &str = formatcp!(r"(\m,n.{IS_ZERO} ({SUB} m n))");
    #[test]
    fn leq() {
        test(formatcp!("{LEQ} {_1} {_2}"), TRUE);
        test(formatcp!("{LEQ} {_2} {_1}"), FALSE);
        test(formatcp!("{LEQ} {_1} {_1}"), TRUE);
    }

    const PAIR: &str = r"(\x,y,f.f x y)";
    const FIRST: &str = formatcp!(r"(\p.p {TRUE})");
    const SECOND: &str = formatcp!(r"(\p.p {FALSE})");
    const NIL: &str = formatcp!(r"(\x.{TRUE})");
    const IS_NULL: &str = formatcp!(r"(\p.p (\x y.{FALSE}))");
    #[test]
    fn pair() {
        const MAPPING: &str = formatcp!(r"(\x.{PAIR} ({SECOND} x) ({SUCC} ({SECOND} x)))");
        test(
            formatcp!("{MAPPING} ({PAIR} {_1} {_4})"),
            formatcp!("{PAIR} {_4} {_5}"),
        )
    }

    #[test]
    fn normal_order_reduction() {
        test(r"(\x.z)((\w.w w w)(\w.w w w))", "z");
    }

    #[test]
    #[should_panic]
    fn non_terminating() {
        test(r"(\x.x x)(\x.x x)", "lol");
    }

    const I: &str = "(λx.x)";
    const K: &str = "(λx.λy.x)";
    const S: &str = "(λx.λy.λz.x z (y z))";
    const B: &str = "(λx.λy.λz.x (y z))";
    const C: &str = "(λx.λy.λz.x z y)";
    const W: &str = "(λx.λy.x y y)";
    const U: &str = "(λx.x x)";
    const O: &str = formatcp!("({U} {U})");
    const Y: &str = "(λg.(λx.g (x x)) (λx.g (x x)))";
    pub fn infinite_recursion() {
        const G: &str = formatcp!(r"(\self, n. self ({SUCC} n))");
        test(formatcp!("({Y} {G}) {_0}"), "lol");
    }
    pub fn factorial() {
        const G: &str = formatcp!(
            r"(    \self, n. ({IF_THEN_ELSE} (({LEQ} n {_0}) {_1} ({MUL} n (self ({PRED} n)))))    )"
        );
        const _24: &str = formatcp!(r"({MUL} {_4} {_6})");
        test(formatcp!("({Y} {G}) {_4}"), _24);
    }
}
