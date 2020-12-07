#![feature(str_split_once)]
use std::{collections::HashMap, hash::Hash};
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let rules: HashMap<String, Vec<(String, usize)>> = get_input();

    // Part A
    let can_contain_gold = rules.iter()
        .filter(|(k, _)| rules.transient_contains(&"shiny gold".to_owned(), k))
        .count();
    println!("{}", can_contain_gold);

    // Part B
    let sum = rules.nested_sum(&"shiny gold".to_owned());
    println!("{}", sum);
}

// just for fun :P
trait TransientContains<K> {
    fn transient_contains(&self, key: &K, start: &K) -> bool;
}
impl<K: Eq + Hash, V: PartialEq> TransientContains<K> for HashMap<K, Vec<(K, V)>> {
    fn transient_contains(&self, key: &K, start: &K) -> bool {
        self.get(&start).unwrap().iter()
            .any(|(c, _)| c == key || self.transient_contains(key, c))
    }
}

trait NestedSum<K> {
    fn nested_sum(&self, key: &K) -> usize;
}
impl<K: Eq + Hash> NestedSum<K> for HashMap<K, Vec<(K, usize)>> {
    fn nested_sum(&self, key: &K) -> usize {
        self.get(key).unwrap().iter()
            .map(|(key2, val)| val * (self.nested_sum(key2)+1))
            .sum()
    }
}


// input parsing
lazy_static!{
    // This conveniently deals with the "contains no other bags" case, too!
    static ref LINE_DELIM: Regex = Regex::new(r"bags?[,.]").unwrap();
}

fn parse_line<'a>(l: &'a str) -> (String, Vec<(String, usize)>) {
    let (color, rest) = l.split_once(" bags contain ").unwrap();
    let contains: Vec<(String, usize)> = LINE_DELIM.split(rest)
        .filter_map(|c| {
            let (num_str, color2) = c.trim().split_once(' ')?;
            Some((color2.to_owned(), num_str.parse().ok()?))
        })
        .collect();
    (color.to_owned(), contains)
}

fn get_input() -> HashMap<String, Vec<(String, usize)>> {
    include_str!("input.txt")
        .lines()
        .map(parse_line)
        .collect()
}
