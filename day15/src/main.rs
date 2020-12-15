use std::collections::HashMap;

fn main() {
    let nums: Vec<usize> = "15,5,1,4,7,0".split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    let mut seen = HashMap::new();
    let mut turn = 0;
    let mut next: usize = 0;

    // initialize with input
    for num in nums.iter() {
        if let Some(last_seen) = seen.insert(*num, turn) {
            next = turn - last_seen;
        }
        else { next = 0; }
        turn += 1;
    }

    // play game
    while turn < 30_000_000 {
        if turn == 30_000_000 - 1 {
            println!("{}", next);
        }
        if let Some(last_seen) = seen.insert(next, turn) {
            next = turn - last_seen;
        }
        else { next = 0; }
        turn += 1;
    }

    /*
        I think there was supposed to be some clever way to do this,
        but this runs in only a couple seconds under a release build
        so I'm not going to bother.
    */
}
