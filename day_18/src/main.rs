use std::env;
use std::str::FromStr;
use std::collections::{HashSet, VecDeque};
use std::ops;
use std::ops::RangeInclusive;
use std::rc::Rc;

// use log::debug;
use itertools::Itertools;
#[macro_use] extern crate impl_ops;

use utils::{read_input, set_logging_level};

#[derive(PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32
}

const TO_SURROUNDING: [Coordinate; 6] = [
    Coordinate {x: 1, y: 0, z: 0},
    Coordinate {x: -1, y: 0, z: 0},
    Coordinate {x: 0, y: 1, z: 0},
    Coordinate {x: 0, y: -1, z: 0},
    Coordinate {x: 0, y: 0, z: 1},
    Coordinate {x: 0, y: 0, z: -1}
];

impl Coordinate {
    fn surroundings(&self) -> std::array::IntoIter<Coordinate, 6> {
        return TO_SURROUNDING.map(|c| self + c).into_iter();
    }
}

impl_op!(
    + |a: &Coordinate, b: Coordinate| -> Coordinate {
        Coordinate { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
    }
);

#[derive(Debug)]
struct ParseCoordinateError;

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s.trim().splitn(3, ',').collect_tuple().ok_or(ParseCoordinateError)?;
        Ok(
            Self {
            x: x.parse::<i32>().map_err(|_| ParseCoordinateError)?,
            y: y.parse::<i32>().map_err(|_| ParseCoordinateError)?,
            z: z.parse::<i32>().map_err(|_| ParseCoordinateError)?
            }
        )
    }
}

fn parse_coordinates(input: &str) -> HashSet<Coordinate> {
    let coordinates: HashSet<Coordinate> = input
        .lines()
        .map(|line| Coordinate::from_str(line).expect("Cannot parse: {line}"))
        .collect();
    return coordinates;
}

fn part_1(coordinates: &HashSet<Coordinate>) -> u32 {
    let mut total_faces: u32 = 0;
    for coordinate in coordinates {
        let mut free_faces: u32 = 6;
        for position in coordinate.surroundings() {
            if coordinates.contains(&position) {
                free_faces -= 1;
            }
        }
        total_faces += free_faces;
    }
    return total_faces;
}

struct Limits {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

impl Limits {
    fn search_limits(coordinates: &Vec<Coordinate>) -> Self {
        let mut x_min: i32 = i32::MAX;
        let mut x_max: i32 = i32::MIN;
        let mut y_min: i32 = i32::MAX;
        let mut y_max: i32 = i32::MIN;
        let mut z_min: i32 = i32::MAX;
        let mut z_max: i32 = i32::MIN;

        for coordinate in coordinates {
            if coordinate.x < x_min {
                x_min = coordinate.x;
            }
            if coordinate.x > x_max {
                x_max = coordinate.x
            }

            if coordinate.y < y_min {
                y_min = coordinate.y;
            }
            if coordinate.y > y_max {
                y_max = coordinate.y
            }

            if coordinate.z < z_min {
                z_min = coordinate.z;
            }
            if coordinate.z > z_max {
                z_max = coordinate.z
            }
        }
        x_min -= 1;
        x_max += 1;
        y_min -= 1;
        y_max += 1;
        z_min -= 1;
        z_max += 1;
        return Self { x: x_min..=x_max, y: y_min..=y_max, z: z_min..=z_max }
    }

    fn contains(&self, coordinate: &Coordinate) -> bool {
        if !self.x.contains(&coordinate.x) {
            return false;
        }
        if !self.y.contains(&coordinate.y) {
            return false;
        }
        if !self.z.contains(&coordinate.z) {
            return false;
        }
        return true;
    }
}

fn part_2(lava_droplets: &Vec<Coordinate>) -> u32 {
    let limits = Limits::search_limits(lava_droplets);
    let starting_point = Rc::new(Coordinate {
        x: *limits.x.start(),
        y: *limits.y.start(),
        z: *limits.z.start()
    });
    let mut queue: VecDeque<Rc<Coordinate>> = VecDeque::from([Rc::clone(&starting_point)]);
    let mut water: HashSet<Rc<Coordinate>> = HashSet::from([starting_point]);

    while !queue.is_empty() {
        let current_position = queue.pop_front().unwrap();
        for coordinate in current_position.surroundings() {
            if !limits.contains(&coordinate) {
                continue;
            }
            if water.contains(&coordinate) || lava_droplets.contains(&coordinate) {
                continue;
            }
            let water_droplet = Rc::new(coordinate);
            queue.push_back(Rc::clone(&water_droplet));
            water.insert(water_droplet);
        }
    }
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let coordinates = parse_coordinates(&input[..]);
    let total_surface_area: u32 = part_1(&coordinates);
    println!("Lava droplet surface area: {total_surface_area}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limits_search_limits() {
        let coordinates = vec![
            Coordinate {x: 1, y: 2, z: 3}
        ];
        let limits = Limits::search_limits(&coordinates);
        assert_eq!(limits.x, 0..=2);
        assert_eq!(limits.y, 1..=3);
        assert_eq!(limits.z, 2..=4);
    }
}
