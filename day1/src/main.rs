use itertools::Itertools;

fn main() {
    let values = include_str!("input.txt")
        .lines()
        .filter_map(|l: &str| l.parse::<isize>().ok())
        .collect::<Vec<_>>();

    /* Part A */
    /*
    for v in values.iter() {
        let difference = 2020 - v;
        if values.contains(&difference) {
            println!("{}", v * difference);
            break;
        }
    }
    */

    let answer = values.iter()
        // Not using combinations_with_replacement() since it allocates, so it's much slower.
        .tuple_combinations()
        .filter_map(|(x, y, z)| {
            let sum = x + y + z;
            let product = x * y * z;
            Some((product, sum))
        })
        .find(|&(_, sum): &(_, isize)| sum == 2020)
        .unwrap().0;
    println!("{}", answer);
}
