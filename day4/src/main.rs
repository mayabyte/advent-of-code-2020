#![feature(str_split_once)]

use std::collections::HashMap;
use regex::Regex;

fn main() {
    let valid_passports = include_str!("input.txt")
        .split_terminator("\n\n")
        .map(|passport| {
            passport.split_ascii_whitespace()
                .map(|field| field.split_once(":").unwrap())
                .collect()
        })
        .map(|passport| is_valid(&passport))
        .filter(|v| *v)
        .count();
    println!("{}", valid_passports);
}

// This isn't my favorite, but imo it's altogether not an awful way of having custom
// validators for each passport field.
static REQUIRED_FIELDS: [(&'static str, fn(&str) -> Option<bool>); 7] = [
    ("byr", |v| {let x = v.parse::<isize>().ok()?; Some(x >= 1920 && x <= 2002 && v.len() == 4)}),
    ("iyr", |v| {let x = v.parse::<isize>().ok()?; Some(x >= 2010 && x <= 2020 && v.len() == 4)}),
    ("eyr", |v| {let x = v.parse::<isize>().ok()?; Some(x >= 2020 && x <= 2030 && v.len() == 4)}),
    ("hgt", |v| {
        let re = Regex::new(r"(?P<height>\d+)(?P<unit>cm|in)").unwrap();
        let m = re.captures_iter(v).next().ok_or("wrong format").ok()?;
        let height: usize = m["height"].parse().ok()?;
        if &m["unit"] == "cm" {
            Some(height >= 150 && height <= 193)
        } else {
            Some(height >= 59 && height <= 76)
        }
    }),
    ("hcl", |v| Some(Regex::new(r"#[\da-fA-F]{6}").unwrap().is_match(v))),
    ("ecl", |v| Some(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v))),
    ("pid", |v| Some(v.len() == 9 && v.parse::<u32>().is_ok()))
];

fn is_valid(fields: &HashMap<&str, &str>) -> bool {
    REQUIRED_FIELDS.iter()
        .all(|(f, validator)| fields.get(f).cloned().map(validator).flatten().unwrap_or(false))
}
