#![feature(bool_to_option)]
use itertools::Itertools;

fn main() {
    let numbers: Vec<u64> = include_str!("input.txt")
        .lines()
        .filter_map(|n| n.parse().ok())
        .collect();

    // Part A: find the first number which *is not* the sum of any two
    // of the previous 25 numbers chosen without replacement.
    let first_noncompliant_number = numbers
        .windows(25+1)
        .find_map(|window| {
            window[0..25].iter()
                .tuple_combinations()
                .map(|(a, b)| a + b)
                .all(|pair_sum| window[25] != pair_sum)
                .then_some(window[25])
        })
        .unwrap();

    // Part B: find the range of contiguous values of any length that
    // all sum to the result of Part A. Answer is the sum of the largest
    // and smallest number in this range.
    let (mut start, mut end) = (0, 2);
    let (smallest, largest) = loop {
        let sum: u64 = numbers[start..end].iter().sum();
        if sum < first_noncompliant_number {
            end += 1;
            continue
        }
        else if sum > first_noncompliant_number {
            start += 1;
            continue
        }
        else {
            break numbers[start..end].iter().minmax().into_option().unwrap()
        }
    };

    println!("{} + {} = {}", smallest, largest, smallest+largest);
}
