use lazy_static::lazy_static;
use regex::{Regex, Captures};
use itertools::Itertools;

fn main() {
    let res_a: usize = include_str!("input.txt")
        .lines()
        .map(|l| add_expr_a(l))
        .sum();
    println!("{}", res_a);

    let res_b: usize = include_str!("input.txt")
        .lines()
        .map(|l| add_expr_b(l))
        .sum();
    println!("{}", res_b);
}

lazy_static!{
    static ref PARENS: Regex = Regex::new(r"\((?P<sub>[^\(\)]+)\)").unwrap();
    static ref ADDITION: Regex = Regex::new(r"(?P<a>\d+) \+ (?P<b>\d+)").unwrap();
}

// Part B
fn add_expr_b(expr: &str) -> usize {
    let mut reduced: String = expr.into();

    // Reduce parenthesized sub-expressions
    while reduced.contains(|c: char| c == '(' || c == ')') {
        reduced = PARENS.replace_all(
            &reduced,
            |caps: &Captures| format!("{}", add_expr_b(&caps["sub"]))
        ).into_owned();
    }

    // Do all the addition first
    while reduced.contains(|c: char| c == '+') {
        reduced = ADDITION.replace_all(
            &reduced,
            |caps: &Captures| format!("{}", &caps["a"].parse::<usize>().unwrap() + &caps["b"].parse::<usize>().unwrap())
        ).into_owned();
    }

    // All the rest will be multiplication, so this is simple
    reduced.split(" * ")
        .map(|n| n.parse::<usize>().unwrap())
        .product()
}

// Part A
fn add_expr_a(expr: &str) -> usize {
    let mut reduced: String = expr.into();

    // Reduce parenthesized sub-expressions
    while reduced.contains(|c: char| c == '(' || c == ')') {
        reduced = PARENS.replace_all(
            &reduced,
            |caps: &Captures| format!("{}", add_expr_a(caps.name("sub").unwrap().as_str()))
        ).into_owned();
    }

    // Apply the operators.
    // This could probably be an iterator but it's easier to think about
    // this way, since seeding the first value is verbose with an iterator.
    let mut terms = reduced.split_whitespace().map(|t| t.trim());
    let mut res: usize = terms.next().unwrap().parse().unwrap();
    for t in terms.chunks(2).into_iter() {
        let t = t.collect_tuple().unwrap();
        match t {
            ("+", n) => res += n.parse::<usize>().unwrap(),
            ("*", n) => res *= n.parse::<usize>().unwrap(),
            _ => panic!("malformed input")
        }
    }

    res
}
