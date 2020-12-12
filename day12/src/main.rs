#![feature(destructuring_assignment)]

fn main() {
    let mut ship = Ship::new();
    for instruction in include_str!("input.txt").lines() {
        ship.move_ship(instruction);
    }
    println!("{}", ship.x.abs() + ship.y.abs());
}

#[derive(Debug)]
struct Ship {
    pub x: i64,
    pub y: i64,
    waypoint_x: i64,
    waypoint_y: i64,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    pub fn move_ship(&mut self, ins: &str) {
        let (dir, amount) = ins.split_at(1);
        let amount: i64 = amount.parse().unwrap();
        match dir {
            "N" => self.waypoint_y += amount,
            "E" => self.waypoint_x += amount,
            "S" => self.waypoint_y -= amount,
            "W" => self.waypoint_x -= amount,
            "R" => for _ in 0..amount/90 { self.rotate_right() },
            "L" => for _ in 0..amount/90 { self.rotate_left() },
            "F" => self.move_f_direction(amount),
            _ => panic!("what")
        }
    }

    // The rotations only come in increments of 90 degrees, so this appraoch is fine.
    // Props to the destructuring assignment feature for making it so concise :P

    fn rotate_right(&mut self) {
        (self.waypoint_x, self.waypoint_y) = (self.waypoint_y, -self.waypoint_x);
    }

    fn rotate_left(&mut self) {
        (self.waypoint_x, self.waypoint_y) = (-self.waypoint_y, self.waypoint_x);
    }

    fn move_f_direction(&mut self, times: i64) {
        self.x += self.waypoint_x * times;
        self.y += self.waypoint_y * times;
    }
}
