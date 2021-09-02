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

fn main() {}

#[cfg(test)]
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

            const ITERATIONS: usize = 100;
            let mut terminated = false;
            for _ in 0..ITERATIONS {
                let last_term = term.clone();
                term = term.b_reduce();
                if last_term == term {
                    terminated = true;
                    break;
                }
                println!("- b_reduce - {term}");
            }
            if !terminated {
                panic!("b_reduce didn't terminate after {} iterations", ITERATIONS)
            }

            loop {
                let last_term = term.clone();
                term = term.n_reduce();
                if last_term == term {
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

    const N0: &str = r"(\f,x.x)";
    const N1: &str = r"(\f,x.f x)";
    const N2: &str = r"(\f,x.f (f x))";
    const N3: &str = r"(\f,x.f (f (f x)))";

    const SUCC: &str = "(λn.λf.λx.f (n f x))";

    const N4: &str = formatcp!("({SUCC} {N3})");
    const N5: &str = formatcp!("({SUCC} {N4})");
    const N6: &str = formatcp!("({SUCC} {N5})");
    const N7: &str = formatcp!("({SUCC} {N6})");
    const N8: &str = formatcp!("({SUCC} {N7})");
    #[test]
    fn succ() {
        test(formatcp!("{SUCC} {N0}"), N1);
        test(formatcp!("{SUCC} ({SUCC} {N0})"), N2);
    }
    const PLUS: &str = formatcp!("(λm.λn.m {SUCC} n)");
    #[test]
    fn plus() {
        test(formatcp!("{PLUS} {N1} {N2}"), N3);
        test(formatcp!("{PLUS} {N2} {N2}"), N4);
    }
    const MULT: &str = formatcp!("(λm.λn.m ({PLUS} n) {N0})");
    #[test]
    fn mult() {
        test(formatcp!("{MULT} {N2} {N3}"), N6);
        test(formatcp!("{MULT} {N4} {N2}"), N8);
    }
    const POW: &str = "(λb.λe.e b)";
    #[test]
    fn pow() {
        test(formatcp!("{POW} {N2} {N3}"), N8);
        test(formatcp!("{POW} {N1} {N3}"), N1);
    }

    const PRED: &str = "(λn.λf.λx.n (λg.λh.h (g f)) (λu.x) (λu.u))";
    const SUB: &str = formatcp!("(λm.λn.n {PRED} m)");
    #[test]
    fn sub() {
        test(formatcp!("{SUB} {N3} {N1}"), N2);
        test(formatcp!("{SUB} {N2} {N5}"), N0);
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

        test(formatcp!("{IF_THEN_ELSE} {TRUE} {N2} {N3}"), N2);
        test(formatcp!("{IF_THEN_ELSE} {FALSE} {N2} {N3}"), N3);
    }
    const IS_ZERO: &str = formatcp!(r"(\n.n (\x.{FALSE}) {TRUE})");
    #[test]
    fn is_zero() {
        test(formatcp!("{IS_ZERO} {N0}"), TRUE);
        test(formatcp!("{IS_ZERO} {N3}"), FALSE);
    }
    const LEQ: &str = formatcp!(r"(\m,n.{IS_ZERO} ({SUB} m n))");
    #[test]
    fn leq() {
        test(formatcp!("{LEQ} {N1} {N2}"), TRUE);
        test(formatcp!("{LEQ} {N2} {N1}"), FALSE);
        test(formatcp!("{LEQ} {N1} {N1}"), TRUE);
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
            formatcp!("{MAPPING} ({PAIR} {N1} {N4})"),
            formatcp!("{PAIR} {N4} {N5}"),
        )
    }

    #[test]
    fn normal_order_reduction() {
        test(r"(\x.z)((\w.w w w)(\w.w w w))", "z");
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
    #[test]
    #[should_panic]
    fn recursion() {
        const G: &str = formatcp!(r"(\self, n. self ({SUCC} n))");
        test(formatcp!("({Y} {G}) {N0}"), "lol");
    }
}
