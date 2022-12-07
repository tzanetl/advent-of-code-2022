use std::env;

use utils::{read_input, set_logging_level};

struct IdRange {
    start: i32,
    end: i32
}

impl IdRange {
    pub fn from_string(input: &str) ->IdRange {
        let split: Vec<&str> = input.split("-").collect();
        IdRange {
            start: split[0].parse::<i32>().unwrap(),
            end: split[1].parse::<i32>().unwrap()
        }
    }

    pub fn contains(&self, other: &IdRange) -> bool {
        let start_out: bool = self.start > other.start;
        let end_out: bool = self.end < other.end;
        return !start_out & !end_out;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let mut fully_contained: i32 = 0;
    let mut split: Vec<&str>;
    let mut range1: IdRange;
    let mut range2: IdRange;

    for line in input.lines() {
        split = line.split(",").collect();
        range1 = IdRange::from_string(split[0]);
        range2 = IdRange::from_string(split[1]);

        if range1.contains(&range2) | range2.contains(&range1) {
            fully_contained += 1;
        }
    }
    println!("Fully contained ranges: {fully_contained}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let r1 = IdRange {start: 3, end: 5};
        let r2 = IdRange {start: 3, end: 4};
        let r3 = IdRange {start: 2, end: 4};
        assert!(r1.contains(&r2));
        assert!(!r2.contains(&r1));
        assert!(r2.contains(&r2));
        assert!(!r2.contains(&r3));
    }
}