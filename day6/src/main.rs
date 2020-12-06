// Part A
// fn main() {
//     let total: usize = include_str!("input.txt")
//         .split_terminator("\n\n")
//         .map(|group| {
//             let mut group = group.chars().filter(|c| *c != '\n').collect::<Vec<_>>();
//             group.sort_unstable();
//             group.dedup();
//             group.len()
//         })
//         .sum();
//     println!("{}", total);
// }

#![feature(iterator_fold_self)]
use std::collections::HashSet;
fn main() {
    let total: usize = include_str!("input.txt")
        .split_terminator("\n\n")
        .map(|group| {
            group.lines()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .fold_first(|a, b| &a & &b) // bitwise AND, a.k.a. set intersection
                .unwrap()
                .len()
        })
        .sum();
    println!("{}", total);
}
