use std::env;
use std::collections::HashSet;

use log::debug;

use utils::{read_input, set_logging_level};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Knot {
    x: isize,
    y: isize
}

struct Bridge {
    knots: Vec<Knot>,
    positions: HashSet<Knot>
}

impl Bridge {
    pub fn new(n_knots: usize) -> Bridge {
        let mut knots: Vec<Knot> = Vec::with_capacity(n_knots);
        for _ in 0..(n_knots) {
            knots.push(Knot { x: 0, y: 0 })
        }

        Bridge {
            knots: knots,
            positions: HashSet::from([Knot { x: 0, y: 0 }])
        }
    }

    fn drag(&mut self, i: usize) {
        let x_distance: isize = self.knots[i - 1].x - self.knots[i].x;
        let y_distance: isize = self.knots[i - 1].y - self.knots[i].y;
        debug!("distance: {}, {}", x_distance, y_distance);

        if (x_distance.abs() < 2) & (y_distance.abs() < 2) {
            return;
        } else if (x_distance.abs() > 1) & (y_distance.abs() == 0) {
            self.knots[i].x += 1 * x_distance.signum();
        } else if (y_distance.abs() > 1) & (x_distance.abs() == 0) {
            self.knots[i].y += 1 * y_distance.signum();
        } else {
            self.knots[i].x += 1 * x_distance.signum();
            self.knots[i].y += 1 * y_distance.signum();
        }
    }

    fn step_up(&mut self) {
        self.knots[0].y += 1;
    }

    fn step_down(&mut self) {
        self.knots[0].y -= 1;
    }

    fn step_left(&mut self) {
        self.knots[0].x -= 1;
    }

    fn step_right(&mut self) {
        self.knots[0].x += 1;
    }

    pub fn process_line(&mut self, line: &str) {
        debug!("{line}");
        let split: Vec<&str> = line.split(" ").collect();

        let step = match split[0] {
            "U" => Bridge::step_up,
            "D" => Bridge::step_down,
            "L" => Bridge::step_left,
            "R" => Bridge::step_right,
            &_ => panic!("Unknown direction: {}", split[0])
        };
        let count: usize = split[1].parse().unwrap();

        for _ in 0..count {
            step(self);

            for i in 1..self.knots.len() {
                self.drag(i);
            }
            debug!("Tail: {}, {}", self.knots.last().unwrap().x, self.knots.last().unwrap().y);
            self.positions.insert(*self.knots.last().unwrap());
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let mut bridge_2 = Bridge::new(2);
    for line in input.lines() {
        bridge_2.process_line(line)
    }
    println!("Visited positions with 2 knots: {}", bridge_2.positions.len());

    let mut bridge_10 = Bridge::new(10);
    for line in input.lines() {
        bridge_10.process_line(line)
    }
    println!("Visited positions with 10 knots: {}", bridge_10.positions.len());
}
