#![feature(str_split_once)]
#![feature(bool_to_option)]
#![feature(iterator_fold_self)]
use std::{collections::{HashMap, HashSet}, ops::RangeInclusive};
use itertools::Itertools;

type Rules = HashMap<&'static str, (RangeInclusive<usize>, RangeInclusive<usize>)>;

fn main() {
    let input = include_str!("input.txt");
    let (rules_str, my_ticket_str, tickets_str) = input.splitn(3, "\n\n").collect_tuple().unwrap();

    let rules: Rules = rules_str.lines()
        .map(|l| {
            let (name, rest) = l.split_once(':').unwrap();
            let (rule1, rule2) = rest.split_once("or").unwrap();
            (name.trim(), (to_range(rule1.trim()), to_range(rule2.trim())))
        })
        .collect();

    let tickets: Vec<Vec<usize>> = tickets_str.lines()
        .skip(1) // the "nearby tickets:" line
        .map(parse_ticket)
        .collect();

    // Part A
    let answer1: usize = tickets.iter()
        .map(|ticket| invalid_fields(&rules, ticket).iter().sum::<usize>())
        .sum();
    println!("{}", answer1);


    // Part B
    let valid_tickets: Vec<Vec<usize>> = tickets_str.lines()
        .skip(1)
        .map(parse_ticket)
        .filter(|ticket| invalid_fields(&rules, ticket).len() == 0)
        .collect();

    // For each field position, what field names could map to it?
    let mut possible_fields: HashMap<usize, HashSet<&str>> = valid_tickets.iter()
        .map(|ticket| ticket.iter().map(|field| valid_for_fields(*field, &rules)).enumerate())
        .fold(HashMap::new(), |mut acc, row| {
            for (i, possible_fields) in row {
                let entry = acc.entry(i).or_insert(possible_fields.clone());
                *entry = &*entry & &possible_fields; // set intersection
            }
            acc
        });

    // 'Shrink' the set by finding field positions that only have one possible field name
    // and removing that name from all other fields' candidate lists.
    loop {
        let (found, uncertain): (Vec<_>, Vec<_>) = possible_fields.values_mut()
            .partition(|v| v.len() == 1);
        if uncertain.len() == 0 { break; }
        let found: HashSet<&str> = found.iter().fold(HashSet::new(), |acc, s| &acc | &**s);
        for v in uncertain {
            *v = &*v - &found;
        }
    }

    // Reify possible_fields into a nicer mapping
    let fields: Vec<&str> = possible_fields.into_iter()
        .map(|(k, v)| (k, v.into_iter().next().unwrap()))
        .sorted_by_key(|(k, _)| *k)
        .map(|(_, v)| v)
        .collect();

    // Now parse my ticket and locate the fields that start with "departure"
    let answer2: usize = parse_ticket(my_ticket_str.lines().nth(1).unwrap())
        .iter()
        .enumerate()
        .map(|(i, val)| (fields.get(i).unwrap(), val))
        .filter(|(field, _)| field.starts_with("departure"))
        .map(|(_, val)| val)
        .product();

    println!("{}", answer2);
}

fn to_range(s: &str) -> RangeInclusive<usize> {
    let (l, r) = s.split_once('-').unwrap();
    (l.parse().unwrap())..=(r.parse().unwrap())
}

fn parse_ticket(s: &str) -> Vec<usize> {
    s.split(',').map(|s| s.parse().unwrap()).collect()
}

fn invalid_fields(rules: &Rules, ticket: &Vec<usize>) -> Vec<usize> {
    ticket.iter()
        .filter_map(|field| {
            let invalid = rules.values()
                .all(|rule| !rule.0.contains(field) && !rule.1.contains(field));
            (invalid).then_some(*field)
        })
        .collect()
}

fn valid_for_fields(val: usize, rules: &Rules) -> HashSet<&str> {
    rules.iter()
        .filter_map(move |(&field_name, (rule1, rule2))| {
            (rule1.contains(&val) || rule2.contains(&val))
            .then_some(field_name)
        })
        .collect()
}
