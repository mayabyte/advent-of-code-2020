#![feature(str_split_once)]
use std::collections::HashMap;
use itertools::Itertools;

// This is extremely lazy. I'm fine with that though since this challenge is boring.

fn main() {
    let mut computer = Computer::new();
    for line in include_str!("input.txt").lines() {
        if line.starts_with("mask") {
            computer.update_mask(line.split_once('=').unwrap().1.trim());
        }
        else if line.starts_with("mem") {
            let (lhs, rhs) = line.split_once('=').unwrap();
            computer.assign(&format!("{:036b}", lhs.trim()[4..(lhs.len()-2)].parse::<u64>().unwrap()), rhs.trim().parse().unwrap());
        }
    }
    println!("{}", computer.sum_all());
}

struct Computer {
    mem: HashMap<u64, u64>,
    mask: String
}

impl Computer {
    pub fn new() -> Self {
        Self {
            mem: HashMap::new(),
            mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_owned()
        }
    }

    pub fn update_mask(&mut self, mask_str: &str) {
        self.mask = mask_str.to_owned();
    }

    pub fn assign(&mut self, index: &str, value: u64) {
        let index: String = index.chars().zip(self.mask.chars())
            .map(|(bit, mask)| {
                match (bit, mask) {
                    (_, '1') => '1',
                    (x, '0') => x,
                    (_, 'X') => 'X',
                    _ => panic!("you broke the rules")
                }
            })
            .collect();
        let num_floating = self.mask.chars().filter(|&c| c == 'X').count();
        for combo in (["1", "0"].repeat(num_floating))
          .iter().combinations(num_floating).unique()
        {
            let mut index = index.to_owned();
            for v in combo {
                index = index.replacen('X', v, 1);
            }
            let final_addr = u64::from_str_radix(&index, 2).unwrap();
            *self.mem.entry(final_addr).or_insert(0) = value;
        }
    }

    pub fn sum_all(&self) -> u64 {
        self.mem.values().sum()
    }
}
