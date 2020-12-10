use std::collections::HashMap;

fn main() {
    let mut joltages: Vec<u64> = include_str!("input.txt")
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();

    // Part A
    // There aren't any duplicates, so we can just sort and take
    // the cumulative difference.

    joltages.push(0);
    joltages.sort_unstable();
    joltages.push(joltages.last().unwrap() + 3);

    let differences = joltages.windows(2)
        .map(|x| x[1] - x[0]);
    let differences_of_1 = differences.clone().filter(|x| *x == 1).count();
    let differences_of_3 = differences.clone().filter(|x| *x == 3).count();

    println!("{}", differences_of_1 * differences_of_3);


    // Part B
    // This is where they try to trick you.
    /*
        This is basically just the naiive solution but 'memoized' with
        a hashmap. This feels like a hack, and I bet it's not the clever
        mathsy solution intended, but if you think about it it's the same
        as doing some fancy cumulative multiplicative sum thing under the
        hood, just expressed differently.
    */
    let mut seen = HashMap::new();
    let res = arrangements(
        *joltages.last().unwrap(),
        &joltages.as_slice()[..joltages.len()-1],
        &mut seen
    );
    println!("{}", res);
}

fn arrangements(a: u64, rest: &[u64], seen: &mut HashMap<u64, u64>) -> u64 {
    if let Some(cached) = seen.get(&a) {
        return *cached;
    }
    if rest.len() == 0 {
        return 1;
    }
    let mut sum = 0;
    for i in rest.len().saturating_sub(3)..rest.len() {
        if a - rest[i] <= 3 {
            sum += arrangements(rest[i], &rest[..i], seen);
        }
    }
    seen.insert(a, sum);
    sum
}
