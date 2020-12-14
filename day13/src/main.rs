#![feature(iterator_fold_self)]

fn main() {
    let mut input = include_str!("input.txt").lines();
    let starting_minute: u64 = input.next().unwrap().parse().unwrap();
    let schedule = input.next().unwrap().split(',')
        .filter(|bus| bus != &"x")
        .map(|bus| bus.parse::<u64>().unwrap());

    // Part A
    let earliest = schedule
        .map(|bus| (bus - (starting_minute % bus), bus))
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap();
    println!("{}", earliest.0 * earliest.1);

    // Part B
    let res = include_str!("input.txt").lines().nth(1).unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, bus)| Some(i as i128).zip(bus.parse::<i128>().ok()))
        .fold_first(|acc, elem| period(acc, elem))
        .unwrap();
        println!("{:?}", res.0);
}

// Given a previous known starting point for the subset of routes considered
// previously, what's the first occurrence and periodicity of the patten after
// lining it up with the next route?
// I.e. the first occurence of a route that takes 17 minutes and must come at
// the 0th minute the offset is 0, and its period is 17. When combined with
// another route with a given start and period - let's say (2, 13) - we get a
// 'combined' start and period, in this case (102, 224). That is, there's a
// repeating pattern of [17, <something>, 13] where the first instance is at
// index 102 and every 224 indices thereafter.
fn period(last: (i128, i128), next: (i128, i128)) -> (i128, i128) {
    let first = (last.0..).step_by(last.1 as usize)
        .filter(move |x| (x+next.0) % next.1 == 0)
        .next().unwrap();
    (first, last.1 * next.1)
}
