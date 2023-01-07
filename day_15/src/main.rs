use std::env;
use std::collections::HashSet;

use log::debug;
use lazy_static::lazy_static;
use regex::Regex;

use utils::{read_input, set_logging_level};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Sensor {
    x: i32,
    y: i32,
    range: u32
}

impl Sensor {
    pub fn from_line(line: &str) -> (Sensor, Beacon) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(-?\d+)").unwrap();
        };
        let mut coordinates = RE.captures_iter(line);
        let x_sensor: i32 = coordinates.next().unwrap()[0].parse().unwrap();
        let y_sensor: i32 = coordinates.next().unwrap()[0].parse().unwrap();
        let x_beacon: i32 = coordinates.next().unwrap()[0].parse().unwrap();
        let y_beacon: i32 = coordinates.next().unwrap()[0].parse().unwrap();
        let range = distance([&x_sensor, &y_sensor], [&x_beacon, &y_beacon]);
        return (Sensor { x: x_sensor, y: y_sensor, range }, Beacon{x: x_beacon, y: y_beacon})
    }

    pub fn in_range(&self, x: &i32, y: &i32) -> bool {
        return self.range >= distance([&self.x, &self.y], [x, y])
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Map {
    sensors: HashSet<Sensor>,
    beacons: HashSet<Beacon>,
    x_min: i32,
    x_max: i32
}

impl Map {
    pub fn from_input(input: &str) -> Map {
        let mut sensors: HashSet<Sensor> = HashSet::new();
        let mut beacons: HashSet<Beacon> = HashSet::new();
        let mut x_min: i32 = i32::MAX;
        let mut x_max: i32 = i32::MIN;

        for line in input.lines() {
            let (sensor, beacon) = Sensor::from_line(line);

            let x_max_test = sensor.x + (sensor.range as i32);
            let x_min_test = sensor.x - (sensor.range as i32);
            if x_max_test > x_max {
                x_max = x_max_test;
            }
            if x_min_test < x_min {
                x_min = x_min_test;
            }

            sensors.insert(sensor);
            beacons.insert(beacon);
        }

        return Map { sensors, beacons, x_min, x_max }
    }

    pub fn no_beacons(&self, y: i32) -> u32 {
        debug!("y={}", y);
        let mut count: u32 = 0;
        for x in self.x_min..=self.x_max {
            for sensor in &self.sensors {
                // If sensor is in range, that tile cannot contain a beacon
                if sensor.in_range(&x, &y) & !self.beacons.contains(&Beacon{ x, y }) {
                    count += 1;
                    debug!("count={}, x={}", count, x);
                    break;
                }
            }
        }
        return count;
    }
}

fn distance(start: [&i32; 2], end: [&i32; 2]) -> u32 {
    return ((start[0] - end[0]).abs() + (start[1] - end[1]).abs()).try_into().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let y: i32;
    if args.contains(&"--test".to_string()) {
        y = 10;
    } else {
        y = 2000000;
    }

    let map = Map::from_input(&input);
    debug!("{:?}", map);
    let count = map.no_beacons(y);
    println!("On line y = {}, {} position(s) cannot contain beacons", y, count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_from_line() {
        let (sensor, beacon) = Sensor::from_line(&"Sensor at x=2, y=18: closest beacon is at x=-2, y=15");
        assert_eq!(sensor, Sensor{ x: 2, y: 18, range: 7});
        assert_eq!(beacon, Beacon{ x: -2, y: 15,});
    }
}
