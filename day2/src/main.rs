#![feature(str_split_once)]

fn main() {
    let passwords = include_str!("input.txt")
        .lines();
    let valid_passwords: usize = passwords
        .filter_map(|l| l.split_once(":"))
        .map(|(policy_str, password)| (Policy::from(policy_str), password.trim()))
        .map(|(policy, password)| policy.check_valid(password))
        .filter(|x| *x)
        .count();
    println!("{}", valid_passwords);
}

struct Policy {
    letter: char,
    min: usize,
    max: usize
}

impl From<&str> for Policy {
    fn from(policy_str: &str) -> Policy {
        let (occurrences_str, letter) = policy_str.split_at(policy_str.len()-1);
        let (min_str, max_str) = occurrences_str.trim().split_once("-").unwrap();
        Policy {
            letter: letter.chars().nth(0).unwrap(),
            min: min_str.parse().unwrap(),
            max: max_str.parse().unwrap()
        }
    }
}

impl Policy {
    // Part A
    // fn check_valid(&self, password: &str) -> bool {
    //     let letter_occurrences = password.chars()
    //         .filter(|c| *c == self.letter)
    //         .count();
    //     letter_occurrences >= self.min && letter_occurrences <= self.max
    // }

    fn check_valid(&self, password: &str) -> bool {
        let letter1 = password.chars().nth(self.min-1).unwrap() == self.letter;
        let letter2 = password.chars().nth(self.max-1).unwrap() == self.letter;

        (letter1 && !letter2) || (letter2 && !letter1)
    }
}
