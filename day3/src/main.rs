fn main() {
    let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let answer: usize = slopes.iter()
        .map(|(x, y)| traverse(*x, *y))
        .product();
    println!("{}", answer);
}

fn traverse(horizontal_step: usize, vertical_step: usize) -> usize {
    include_str!("input.txt").lines()
        .step_by(vertical_step)
        .enumerate()
        .filter_map(|(i, row)| row.chars().cycle().nth(i*horizontal_step))
        .filter(|space| *space == '#')
        .count()
}
