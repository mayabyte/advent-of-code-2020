#![feature(str_split_once)]
#![feature(bool_to_option)]
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input_part_2.txt");
    let (rules, strings) = input.split_once("\n\n").unwrap();

    // Using a hashmap since one of the sample inputs isn't contiguous like the real one
    let rules: HashMap<usize, Rule> = rules.lines()
        .map(parse_rule)
        .sorted_by_key(|(idx, _)| *idx)
        .collect();

    let num_matching = strings.lines()
        // This flat_map is Part B. Remove it to get Part A.
        // I exploit the fact that rule 8 is the hard one to parse since it 'branches';
        // it can consume some strings a variable number of times.
        .flat_map(|s| {
            let mut rule_8s: Vec<&str> = Vec::new();
            while let Some(app) = matches(rule_8s.last().unwrap_or(&s), 42, &rules) {
                rule_8s.push(app);
            }
            rule_8s
        })

        .filter(|s| matches(s, 11, &rules)
            .map(|remaining| remaining.len() == 0)
            .unwrap_or(false)
        )
        .count();
    println!("{}", num_matching);
}

fn matches<'a>(s: &'a str, rule_i: usize, rules: &HashMap<usize, Rule>) -> Option<&'a str> {
    let rule = rules.get(&rule_i).unwrap();
    let check_rules = |r: &Vec<usize>| {
        r.iter().fold(Some(s), |acc, r| matches(acc?, *r, rules))
    };

    match rule {
        Rule::Lit(c) => s.starts_with(c).then_some(s.get(1..)).flatten(),
        Rule::Ref(r1, None) => check_rules(r1),
        Rule::Ref(r1, Some(r2)) => check_rules(r1).or(check_rules(r2))
    }
}

#[derive(Debug)]
enum Rule {
    Lit(String),
    Ref(Vec<usize>, Option<Vec<usize>>)
}

fn parse_rule(rule_str: &str) -> (usize, Rule) {
    let (idx, body) = rule_str.split_once(':').unwrap();
    let idx: usize = idx.parse().unwrap();
    let body = body.trim();

    if body.starts_with('\"') {
        (
            idx,
            Rule::Lit(body.trim_matches('\"').to_owned())
        )
    }
    else {
        let mut rules = body.split('|')
            .filter(|r| r.len() > 0)
            .map(|r| r.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect());
        (
            idx,
            Rule::Ref(rules.next().unwrap(), rules.next())
        )
    }

}
