fn main() {
    let mut seats = include_str!("input.txt")
        .lines()
        .map(|code| code.split_at(7))
        .map(|(row_code, col_code)| seat_id(row(row_code), col(col_code)))
        .collect::<Vec<_>>();
    seats.sort_unstable();
    let my_seat = seats.windows(2)
        .find(|x| x[0]+1 != x[1])
        .unwrap()[0] + 1;
    println!("{}", my_seat);
}

fn row(code: &str) -> usize {
    let mut range = 0..127;
    for c in code.chars() {
        match c {
            'F' => { range = range.start..(range.start+range.len()/2) },
            'B' => { range = (range.start+range.len()/2 + 1)..range.end },
            _ => panic!("what")
        }
    };
    range.start
}

fn col(code: &str) -> usize {
    let mut range = 0..7;
    for c in code.chars() {
        match c {
            'L' => { range = range.start..(range.start+range.len()/2) },
            'R' => { range = (range.start+range.len()/2 + 1)..range.end },
            _ => panic!("what")
        }
    };
    range.start
}

fn seat_id(row: usize, col: usize) -> usize {
    (row * 8) + col
}
