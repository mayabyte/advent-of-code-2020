#![feature(bool_to_option)]
use std::{collections::{BTreeMap, BTreeSet}, fmt::{self, Display, Formatter}};
use itertools::Itertools;

// I'm sure there's a nicer, more general way to do this, but unless this type
// of problem recurs later I don't think I'll implement it. Sounds fun though.

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();
    let mut cube = ConwayCube::new(input);

    //println!("{}", cube);
    for _ in 0..6 {
        cube = cube.step();
        // println!("Iter {}: \n{}", i+1, cube);
    }
    println!("{}", cube.count());
}


#[derive(Clone)]
struct ConwayCube {
    spaces: BTreeMap<isize, BTreeMap<isize, BTreeMap<isize, BTreeSet<isize>>>>,
}

impl ConwayCube {
    pub fn new(plane: Vec<Vec<bool>>) -> Self {
        let mut cube = Self {
            spaces: BTreeMap::new(),
        };
        for (i, x) in plane.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                cube.set((0, 0, i as isize, j as isize), *y);
            }
        }
        cube
    }

    pub fn step(&self) -> Self {
        let mut new = self.clone();

        let planes = self.spaces.keys().minmax().into_option().unwrap();
        let lines = self.spaces.values().flat_map(|p| p.keys()).minmax().into_option().unwrap();
        let spots = self.spaces.values().flat_map(|p| p.values().flat_map(|l| l.keys())).minmax().into_option().unwrap();
        let hyper = self.spaces.values().flat_map(|p| p.values().flat_map(|l| l.values().flat_map(|w| w.iter()))).minmax().into_option().unwrap();

        for x in ((planes.0-1)..=(planes.1+1)).into_iter() {
            for y in ((lines.0-1)..=(lines.1+1)).into_iter() {
                for z in ((spots.0-1)..=(spots.1+1)).into_iter() {
                    for w in ((hyper.0-1)..=(hyper.1+1)).into_iter() {
                        let neighbors = self.count_neighbors((x,y,z,w));
                        let is_alive = self.get((x,y,z,w));
                        match neighbors {
                            3 => new.set((x,y,z,w), true),
                            2 if is_alive => { /* it stays alive */ },
                            _ => new.set((x,y,z,w), false)
                        };
                    }
                }
            }
        }
        new
    }

    pub fn count(&self) -> usize {
        self.spaces.iter()
            .map(|(_, plane)| plane.iter()
                .map(|(_, line)| line.iter()
                    .map(|(_, hyper)| hyper.iter().count())
                    .sum::<usize>()
                ).sum::<usize>()
            ).sum()
    }

    fn get(&self, index: (isize, isize, isize, isize)) -> bool {
        self.spaces.get(&index.0).iter()
            .flat_map(|p| p.get(&index.1))
            .flat_map(|l| l.get(&index.2))
            .map(|h| h.contains(&index.3))
            .next()
            .unwrap_or_default()
    }

    fn set(&mut self, index: (isize, isize, isize, isize), val: bool) {
        let line = self.spaces.entry(index.0)
            .or_default()
            .entry(index.1)
            .or_default()
            .entry(index.2)
            .or_default();
        if val { line.insert(index.3); }
        else { line.remove(&index.3); }
    }

    pub fn count_neighbors(&self, index: (isize, isize, isize, isize)) -> usize {
        let mut res = self.spaces.range((index.0-1)..=(index.0+1))
            .map(|(_, y)| y.range((index.1-1)..=(index.1+1))
                .map(|(_, z)| z.range((index.2-1)..=(index.2+1))
                    .map(|(_, w)| w.range((index.3-1)..=(index.3+1)).count())
                    .sum::<usize>()
                ).sum::<usize>()
            ).sum();
        if self.get(index) { res -= 1; }
        res
    }
}

// Commented this out since I don't feel like updating it for the 4D case
// impl Display for ConwayCube {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         let planes = self.spaces.keys().minmax().into_option().unwrap();
//         let lines = self.spaces.values().flat_map(|p| p.keys()).minmax().into_option().unwrap();
//         let spots = self.spaces.values().flat_map(|p| p.values().flat_map(|l| l.iter())).minmax().into_option().unwrap();
//         for x in (*planes.0..=*planes.1).into_iter() {
//             write!(f, "z={}\n", x)?;
//             for y in (*lines.0..=*lines.1).into_iter() {
//                 for z in (*spots.0..=*spots.1).into_iter() {
//                     if self.get((x,y,z)) { write!(f, "#")?; }
//                     else { write!(f, ".")?; }
//                 }
//                 write!(f, "\n")?;
//             }
//             write!(f, "\n")?;
//         }
//         Ok(())
//     }
// }
