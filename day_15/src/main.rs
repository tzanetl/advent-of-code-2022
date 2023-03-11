use std::{env, time::Instant};
use std::ops::RangeInclusive;
use std::collections::HashSet;
use std::thread;
use std::sync::mpsc;

use log::debug;
use lazy_static::lazy_static;
use regex::Regex;

use utils::{read_input, set_logging_level};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beacon {
    x: i32,
    y: i32
}

#[derive(Debug, Clone)]
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

    pub fn width(&self) -> u32 {
        return self.x_min.abs_diff(self.x_max)
    }

    fn covers(&self, sensor: &Sensor, y: &i32) -> Option<RangeInclusive<usize>> {
        let width: i32 = (sensor.range as i32 * 2 + 1) - (y.abs_diff(sensor.y) * 2) as i32;
        if width < 1 {
            return None
        }
        let offset: i32 = ((width - 1) / 2) as i32;
        let start: usize = match (sensor.x - offset + self.x_min.abs()).try_into() {
            Ok(value) => value,
            Err(_) => self.x_min.try_into().unwrap()
        };
        let mut end: usize = (sensor.x + offset + self.x_min.abs()).try_into().unwrap();
        if end >= self.width() as usize {
            end = (self.width() - 1) as usize;
        }
        return Some(RangeInclusive::new(start, end))
    }

    fn covered(&self, y: i32) -> Vec<bool> {
        debug!("y={}", y);
        let mut row = vec![true; self.width() as usize];
        debug!("{:?}", row.len());
        for sensor in &self.sensors {
            if let Some(sensor_range) = self.covers(sensor, &y) {
                debug!("{:?}", sensor_range);
                for i in sensor_range {
                    row[i] = false;
                }
            }
        }
        return row;
    }

    pub fn no_beacons(&self, y: i32) -> usize {
        let mut row = self.covered(y);

        for beacon in &self.beacons {
            if beacon.y == y {
                row[(beacon.x + self.x_min.abs()) as usize] = true;
            };
        };
        debug!("{:?}", &row);
        debug!("{:?}", row.len());
        return row.iter().filter(|&n| *n == false).count()
    }

}

fn beacon_frequency(map: Map) -> Option<i32>{
    let threads: i32 = 1;
    println!("Using {} threads", threads);
    let mut bounds: Vec<i32> = Vec::new();
    let step = map.x_max.abs_diff(map.x_min) as i32 / threads;

    for i in 0..threads {
        bounds.push(map.x_min + i * step);
    }
    bounds.push(map.x_max);
    debug!("Bounds: {:?}", bounds);

    let (tx, rx) = mpsc::channel();

    for t in 0..(threads as usize) {
        let map_clone = map.clone();
        let tx_clone = tx.clone();
        let start = bounds[t];
        let end = bounds[t + 1];
        thread::spawn(move || {
            match find_in_range(map_clone, start, end) {
                Some(point) => tx_clone.send(Some(point)).expect("Unable to return point"),
                None => match tx_clone.send(None) {
                    Ok(_) => println!("Thread returned ok"),
                    Err(_) => println!("Thread didn't return ok")
                }
                
            }
        });
    }

    for received in rx {
        match received {
            Some(point) => {
                println!("Point: {:?}", point);
                return Some(point[0] * 4000000 + point[1])
            },
            None => continue
        }
    }
    return None;
}

fn find_in_range(map: Map, start: i32, end: i32) -> Option<[i32; 2]> {
    for y in start..=end {
        let covered = map.covered(y);
        if covered.contains(&true) {
            let x: i32 = covered.iter().position(|&v| v == true).unwrap() as i32;
            debug!("Range {}..={}", start, end);
            debug!("Possible beacon x={} y={}", x, y);
            return Some([x, y]);
        }
    }
    debug!("No empty positions found in rows {} to {}", start, end);
    return None;
}

fn distance(start: [&i32; 2], end: [&i32; 2]) -> u32 {
    return (start[0].abs_diff(*end[0]) + start[1].abs_diff(*end[1])).try_into().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let y: i32;
    let min_range: i32 = 0;
    let max_range: i32;
    if args.contains(&"--test".to_string()) {
        y = 10;
        max_range = 20;
    } else {
        y = 2000000;
        max_range = 4000000;
    }

    let mut map = Map::from_input(&input);
    debug!("{:?}", map);

    // Part 1
    let start = Instant::now();
    let count = map.no_beacons(y);
    let duration = start.elapsed();
    println!("On line y = {}, {} position(s) cannot contain beacons", y, count);
    println!("Part 1 took: {:?}", duration);

    // Part 2
    // 666405357
    // "correct"
    // found solution (2889605, 3398893)
    // answer 11558423398893

    // Todo remove these
    map.x_min = min_range;
    map.x_max = max_range;
    let frequency = beacon_frequency(map).unwrap();
    println!("Tuning frequency is {}", frequency);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_from_line() {
        let (sensor, beacon) = Sensor::from_line(
            &"Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        );
        assert_eq!(sensor, Sensor{ x: 2, y: 18, range: 7});
        assert_eq!(beacon, Beacon{ x: -2, y: 15,});
    }

    #[test]
    fn test_beacon_frequency() {
        let args = vec!["--test".to_string()];
        let input = read_input(&args);
        set_logging_level(&args);
        let mut map = Map::from_input(&input);
        map.x_min = 10;
        map.x_max = 15;
        let frequency = beacon_frequency(map).unwrap();
        println!("{}", frequency);
    }

    #[test]
    fn test_sensor_no_beacons_y_9() {
        let input = read_input(&vec!["--test".to_string()]);
        let map = Map::from_input(&input);
        let count = map.no_beacons(9);
        assert!(count == 25);
    }

    #[test]
    fn test_sensor_no_beacons_y_10() {
        let input = read_input(&vec!["--test".to_string()]);
        let map = Map::from_input(&input);
        let count = map.no_beacons(10);
        assert!(count == 26);
    }

    #[test]
    fn test_sensor_no_beacons_y_11() {
        let args = vec!["--test".to_string()];
        // set_logging_level(&args);
        let input = read_input(&args);
        let map = Map::from_input(&input);
        let count = map.no_beacons(11);
        assert_eq!(count, 28);
    }

    #[test]
    fn test_sensor_no_beacons_y_6() {
        let args = vec!["--test".to_string()];
        // set_logging_level(&args);
        let input = read_input(&args);
        let map = Map::from_input(&input);
        let count = map.no_beacons(6);
        assert_eq!(count, 25);
    }

    #[test]
    fn test_sensor_no_beacons_example_1_row_7() {
        // let args = vec!["--test".to_string()];
        // set_logging_level(&args);
        let map = Map {
            beacons: HashSet::new(),
            sensors: HashSet::from([Sensor{ x: 8, y: 7, range: 9}]),
            x_min: -2,
            x_max: 25
        };
        let count = map.no_beacons(7);
        assert_eq!(count, 19)
    }

    #[test]
    fn test_sensor_no_beacons_example_1_row_6() {
        // let args = vec!["--test".to_string()];
        // set_logging_level(&args);
        let map = Map {
            beacons: HashSet::new(),
            sensors: HashSet::from([Sensor{ x: 8, y: 7, range: 9}]),
            x_min: -2,
            x_max: 25
        };
        let count = map.no_beacons(6);
        assert_eq!(count, 17)
    }

    #[test]
    fn test_sensor_no_beacons_example_1_row_19() {
        // let args = vec!["--test".to_string()];
        // set_logging_level(&args);
        let map = Map {
            beacons: HashSet::new(),
            sensors: HashSet::from([Sensor{ x: 8, y: 7, range: 9}]),
            x_min: -2,
            x_max: 25
        };
        let count = map.no_beacons(19);
        assert_eq!(count, 0)
    }

    #[test]
    fn test_sensor_no_beacons_example_1_row_ne2() {
        // let args = vec!["--test".to_string()];
        // set_logging_level(&args);
        let map = Map {
            beacons: HashSet::new(),
            sensors: HashSet::from([Sensor{ x: 8, y: 7, range: 9}]),
            x_min: -2,
            x_max: 25
        };
        let count = map.no_beacons(-2);
        assert_eq!(count, 1)
    }

    #[test]
    fn test_sensor_no_beacons_example_1_row_ne1() {
        // let args = vec!["--test".to_string()];
        // set_logging_level(&args);
        let map = Map {
            beacons: HashSet::new(),
            sensors: HashSet::from([Sensor{ x: 8, y: 7, range: 9}]),
            x_min: -2,
            x_max: 25
        };
        let count = map.no_beacons(-1);
        assert_eq!(count, 3)
    }
}
