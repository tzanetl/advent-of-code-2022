use std::env;
use std::collections::HashMap;
use std::time::Instant;

use log::debug;

use utils::{read_input, set_logging_level};

#[derive(PartialEq, Eq)]
enum Tile {
    Wall,
    Sand
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coordinate {
    x: u32,
    y: u32
}

impl Coordinate {
    pub fn from_string(input: &str) -> Coordinate {
        let mut split = input.split(",");
        Coordinate {
            x: split.next().unwrap().parse::<u32>().unwrap(),
            y: split.next().unwrap().parse::<u32>().unwrap()
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn create_points(line: &str) -> Vec<Coordinate> {
    let mut path: Vec<Coordinate> = Vec::new();
    for split in line.split(" -> ") {
        path.push(Coordinate::from_string(split))
    }
    return path;
}

fn path_direction(start_point: &Coordinate, end_point: &Coordinate) -> Direction {
    if start_point.x == end_point.x {
        if start_point.y > end_point.y {
            Direction::Up
        } else {
            Direction::Down
        }
    } else {
        if start_point.x > end_point.x {
            Direction::Left
        } else {
            Direction::Right
        }
    }
}

fn create_map(input: &str) -> HashMap<Coordinate, Tile> {
    let mut map: HashMap<Coordinate, Tile> = HashMap::new();

    for line in input.lines() {
        let points = create_points(line);

        for i in 1..points.len() {
            let start_point = &points[i - 1];
            let end_point = &points[i];

            let direction = path_direction(start_point, end_point);

            match direction {
                Direction::Up => {
                    for y in (end_point.y + 1)..=start_point.y {
                        map.insert(Coordinate { x: start_point.x, y }, Tile::Wall);
                    }
                },
                Direction::Down => {
                    for y in start_point.y..end_point.y {
                        map.insert(Coordinate { x: start_point.x, y }, Tile::Wall);
                    }
                },
                Direction::Left => {
                    for x in (end_point.x + 1)..=start_point.x {
                        map.insert(Coordinate { x, y: start_point.y }, Tile::Wall);
                    }
                },
                Direction::Right => {
                    for x in start_point.x..end_point.x {
                        map.insert(Coordinate { x, y: start_point.y }, Tile::Wall);
                    }
                }
            }
            debug!("\n{}", print_map(&map, [494, 503], [0, 9]));
        }
        let last_point = points.last().unwrap();
        map.insert(*last_point, Tile::Wall);
        debug!("\n{}", print_map(&map, [494, 503], [0, 9]));
    }
    debug!("\n{}", print_map(&map, [494, 503], [0, 9]));
    return map;
}

fn print_map(map: &HashMap<Coordinate, Tile>, x_range: [u32; 2], y_range: [u32; 2]) -> String{
    let mut row: String = String::new();
    for y in y_range[0]..=y_range[1] {
        row.push_str(&format!("{:<3}", y)[..]);
        for x in x_range[0]..=x_range[1] {
            match map.get(&Coordinate{ x, y }) {
                None => row.push('.'),
                Some(tile) => {
                    match tile {
                        Tile::Sand => row.push('o'),
                        Tile::Wall => row.push('#')
                    }
                }
            }
        }
        if y != y_range[1] {
            row.push('\n');
        }
    }
    return row
}

fn lowest_point(map: &HashMap<Coordinate, Tile>, x: u32, from_y: u32) -> Option<u32> {
    let mut keys_filtered: Vec<&Coordinate> = map.keys().into_iter()
        .filter(|c| (c.x == x) & (c.y > from_y))
        .collect();

    if keys_filtered.len() == 0 {
        return None;
    }
    keys_filtered.sort_unstable_by_key(|c| c.y);

    return Some(keys_filtered[0].y - 1);
}

fn drop_sand(mut map: HashMap<Coordinate, Tile>) -> (HashMap<Coordinate, Tile>, u32) {
    let mut x: u32;
    let mut y: u32;
    let mut sand_dropped: u32 = 0;

    loop {
        x = 500;
        y = 0;

        loop {
            y = match lowest_point(&map, x, y) {
                Some(value) => value,
                None => return (map, sand_dropped)
            };
            if map.get(&Coordinate{x: x - 1, y: y + 1}) == None {
                x -= 1;
                continue;
            }
            if map.get(&Coordinate{x: x + 1, y: y + 1}) == None {
                x += 1;
                continue;
            }
            map.insert(Coordinate{x, y}, Tile::Sand);
            break;
        }
        sand_dropped += 1;
        debug!("sand dropped: {sand_dropped}");
        debug!("\n{}", print_map(&map, [494, 503], [0, 9]));
    }
}

fn drop_sand_to_floor(mut map: HashMap<Coordinate, Tile>, floor_y: u32)
    -> (HashMap<Coordinate, Tile>, u32) {
    let mut x: u32;
    let mut y: u32;
    let mut sand_dropped: u32 = 0;

    loop {
        x = 500;
        y = 0;

        loop {
            y = match lowest_point(&map, x, y) {
                Some(value) => value,
                None => {
                    map.insert(Coordinate {x, y: floor_y}, Tile::Sand);
                    break;
                }
            };
            if map.get(&Coordinate{x: x - 1, y: y + 1}) == None {
                x -= 1;
                continue;
            }
            if map.get(&Coordinate{x: x + 1, y: y + 1}) == None {
                x += 1;
                continue;
            }
            map.insert(Coordinate{x, y}, Tile::Sand);

            if (x == 500) & (y == 0) {
                sand_dropped += 1;
                return (map, sand_dropped);
            }
            break;
        }
        sand_dropped += 1;
        debug!("sand dropped: {sand_dropped}");
        debug!("\n{}", print_map(&map, [494, 503], [0, 9]));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let map = create_map(&input);
    let now = Instant::now();
    let (map, sand_count) = drop_sand(map);
    let elapsed_time = now.elapsed();
    println!("Part 1 took: {:?}", elapsed_time);
    debug!("{}", print_map(&map, [494, 503], [0, 9]));
    println!("Sand dropped before overflow: {sand_count}");

    // Part 2
    let mut keys: Vec<&Coordinate> = map.keys().collect();
    keys.sort_by_key(|c| c.y);
    let floor_y: u32 = keys.last().unwrap().y + 1;
    let (map, sand_count_to_floor) = drop_sand_to_floor(map, floor_y);
    debug!("\n{}", print_map(&map, [488, 515], [0, 10]));
    println!("Sand dropped before filling up: {}", sand_count + sand_count_to_floor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_map() {
        let input = read_input(&vec!["--test".to_string()]);
        let map = create_map(&input);
        let map_string = print_map(&map, [494, 503], [0, 9]);
        let correct ="\
        0  ..........\n\
        1  ..........\n\
        2  ..........\n\
        3  ..........\n\
        4  ....#...##\n\
        5  ....#...#.\n\
        6  ..###...#.\n\
        7  ........#.\n\
        8  ........#.\n\
        9  #########.".to_string();
        assert_eq!(map_string, correct);
    }

    #[test]
    fn test_lowest_point_found() {
        // set_logging_level(&vec!["--test".to_string()]);
        let input = read_input(&vec!["--test".to_string()]);
        let map = create_map(&input);
        let lowest_point = lowest_point(&map, 497, 0);
        assert_eq!(lowest_point, Some(5));
    }

    #[test]
    fn test_lowest_point_found_from_9() {
        // set_logging_level(&vec!["--test".to_string()]);
        let input = read_input(&vec!["--test".to_string()]);
        let map = create_map(&input);
        let lowest_point = lowest_point(&map, 497, 6);
        assert_eq!(lowest_point, Some(8));
    }

    #[test]
    fn test_lowest_point_not_found() {
        // set_logging_level(&vec!["--test".to_string()]);
        let input = read_input(&vec!["--test".to_string()]);
        let map = create_map(&input);
        let lowest_point = lowest_point(&map, 493, 0);
        assert_eq!(lowest_point, None);
    }
}
