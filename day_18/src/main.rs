use std::env;
use std::str::FromStr;
use std::collections::{HashSet};
use std::ops;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let coordinates = parse_coordinates(&input[..]);
    let total_surface_area: u32 = part_1(&coordinates);
    println!("Lava droplet surface area: {total_surface_area}");
}
