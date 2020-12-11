#![feature(destructuring_assignment)]
use std::fmt::{Display, Formatter};

// I'm really tired today, so I didn't make much effort to
// make this solution pretty. It works though!

fn main() {
    let mut seats = Seats::load();
    let finished = loop {
        let new_seats = seats.step();
        if new_seats == seats {
            break new_seats
        }
        else {
            seats = new_seats;
        }
    };
    println!("{}", finished.total_occupied_seats());
}

#[derive(PartialEq, Clone)]
struct Seats {
    map: Vec<Vec<char>>,
}

impl Seats {
    pub fn load() -> Self {
        let map = include_str!("input.txt")
            .lines()
            .map(|l| l.chars().collect())
            .collect();
        Self {
            map
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || y < 0 { return None; }
        self.map.get(y as usize)?.get(x as usize).cloned()
    }

    fn find_occupied(&self, mut x: isize, mut y: isize, direction: fn(isize, isize) -> (isize, isize)) -> bool {
        (x, y) = direction(x, y); // don't check the starting point itself!
        while let Some(seat) = self.get(x, y) {
            match seat {
                '#' => return true,
                'L' => return false,
                _ => { (x, y) = direction(x, y); }
            }
        };
        false
    }

    fn num_adjacent(&self, x: isize, y: isize) -> usize {
        SEAT_DIRECTION_FNS.iter()
            .map(|func| self.find_occupied(x, y, *func))
            .filter(|s| *s)
            .count()
    }

    pub fn step(&self) -> Self {
        let new_map = self.map.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, seat)| {
                        let num_occupied = self.num_adjacent(x as isize, y as isize);
                        match *seat {
                            'L' if num_occupied == 0 => '#',
                            '#' if num_occupied >= 5 => 'L',
                            other => other
                        }
                    })
                    .collect()
            })
            .collect();
        Self {
            map: new_map
        }
    }

    pub fn total_occupied_seats(&self) -> usize {
        self.map.iter()
            .map(|row| row.iter().filter(|seat| **seat == '#').count())
            .sum()
    }
}

impl Display for Seats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.map.iter()
            .map(|row| row.iter()
                .map(|c| write!(f, "{}", c))
                .collect::<std::fmt::Result>()
                .and_then(|_| write!(f, "\n")))
            .collect()
    }
}

const SEAT_DIRECTION_FNS: [fn(isize, isize) -> (isize, isize); 8] = [
    |x, y| (x-1, y-1),
    |x, y| (x, y-1),
    |x, y| (x+1, y-1),
    |x, y| (x-1, y),
    |x, y| (x+1, y),
    |x, y| (x-1, y+1),
    |x, y| (x, y+1),
    |x, y| (x+1, y+1)
];
