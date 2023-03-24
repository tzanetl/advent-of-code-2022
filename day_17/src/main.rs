use std::env;
use std::iter;

#[macro_use] extern crate impl_ops;
use std::ops;

use utils::{read_input, set_logging_level};

use indicatif::ProgressIterator;

const MAP_WIDTH: i32 = 7;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right
}

struct JetPattern <'a>{
    directions: iter::Cycle<core::str::Chars<'a>>
}

impl<'a> JetPattern <'a>{
    fn new(inp: &'a str) -> Self {
        return Self {
            directions: inp.trim().chars().cycle()
        };
    }

    fn next_direction(&mut self) -> Direction {
        return match &self.directions.next().unwrap() {
            '<' => {Direction::Left},
            '>' => {Direction::Right},
            no_match => panic!("Non matching character '{no_match}'")
        };
    }
}

struct Coordinate {
    // Vertical position (columns)
    x: i32,
    // Horizontal position (rows)
    y: i32
}

// use impl_op_ex if more combinations of borrowed/owned varians are needed
impl_op!(
    + |a: &Coordinate, b: &Coordinate| -> Coordinate { Coordinate { x: a.x + b.x, y: a.y + b.y }}
);

struct Rock {
    body: Vec<Coordinate>,
    height: usize
}

impl Rock {
    fn bottoms_out(&self, map: &Vec<Vec<bool>>, position: &Coordinate) -> bool {
        for block in &self.body {
            let mut check_position: Coordinate = position + block;
            check_position.y -= 1;

            if map[check_position.y as usize][check_position.x as usize] {
                return true;
            }
        };
        return false;
    }

    fn collides_right(&self, map: &Vec<Vec<bool>>, position: &Coordinate) -> bool {
        for block in &self.body {
            let mut check_position: Coordinate = position + block;
            check_position.x += 1;

            if !(check_position.x < MAP_WIDTH) {
                return true;
            }

            if map[check_position.y as usize][check_position.x as usize] {
                return true;
            }
        };
        return false;
    }

    fn collides_left(&self, map:&Vec<Vec<bool>>, position: &Coordinate) -> bool {
        for block in &self.body {
            let mut check_position: Coordinate = position + block;

            if check_position.x == 0 {
                return true;
            }

            check_position.x -= 1;

            if map[check_position.y as usize][check_position.x as usize] {
                return true;
            }
        };
        return false;
    }

    fn wide_rock() -> Self {
        Self {
            body: vec![
                Coordinate{x: 0, y: 0},
                Coordinate{x: 1, y: 0},
                Coordinate{x: 2, y: 0},
                Coordinate{x: 3, y: 0},
            ],
            height: 1
        }
    }

    fn plus_rock() -> Self {
        Self {
            body: vec![
                Coordinate {x: 1, y: 0},
                Coordinate {x: 0, y: -1},
                Coordinate {x: 1, y: -1},
                Coordinate {x: 2, y: -1},
                Coordinate {x: 1, y: -2}
            ],
            height: 3
        }
    }

    fn l_rock() -> Self {
        Self {
            body: vec![
                Coordinate {x: 2, y: 0},
                Coordinate {x: 2, y: -1},
                Coordinate {x: 0, y: -2},
                Coordinate {x: 1, y: -2},
                Coordinate {x: 2, y: -2}
            ],
            height: 3
        }
    }

    fn tall_rock() -> Self {
        Self {
            body: vec![
                Coordinate {x: 0, y: 0},
                Coordinate {x: 0, y: -1},
                Coordinate {x: 0, y: -2},
                Coordinate {x: 0, y: -3}
            ],
            height: 4
        }
    }

    fn square_rock() -> Self {
        Self {
            body: vec![
                Coordinate {x: 0, y: 0},
                Coordinate {x: 1, y: 0},
                Coordinate {x: 0, y: -1},
                Coordinate {x: 1, y: -1}
            ],
            height: 2
        }
    }
}


trait RockMap {
    fn add_rows(&mut self, rock_height: &usize) -> Coordinate;
    fn add_rock(&mut self, rock: &Rock, position: &Coordinate);
    fn print_map(&self, tail: Option<usize>);
    fn pile_height(&self) -> usize;
}

impl RockMap for Vec<Vec<bool>> {
    fn add_rows(&mut self, rock_height: &usize) -> Coordinate {
        let mut n_free_rows: i32 = 0;
        let mut row: usize = self.len();

        loop {
            row -= 1;
            if self[row].iter().any(|&x| x) {
                break;
            }
            n_free_rows += 1;
        }

        let difference: i32 = n_free_rows - (*rock_height as i32 + 3);

        if difference == 0 {
            return Coordinate {x: 2, y: self.len() as i32 - 1};
        } else if difference < 0 {
            self.append(&mut vec![vec![false; MAP_WIDTH as usize]; (difference * -1) as usize]);
            return Coordinate {x: 2, y: self.len() as i32 - 1};
        // Aka difference > 1
        } else {
            return Coordinate {x: 2, y: self.len() as i32 - 1 - difference};
        }
    }

    fn add_rock(&mut self, rock: &Rock, position: &Coordinate) {
        for block in &rock.body {
            let block_position = position + &block;
            self[block_position.y as usize][block_position.x as usize] = true;
        }
    }

    fn print_map(&self, tail: Option<usize>) {
        println!("");
        for (i, row) in self.iter().rev().enumerate() {
            if let Some(limit) = tail {
                if limit == i {
                    return;
                }
            }
            let mut row_s: String = "".to_owned();
            for block in row {
                if *block {
                    row_s.push('#');
                } else {
                    row_s.push('.');
                }
            }
            println!("{row_s}");
        }
    }

    fn pile_height(&self) -> usize {
        let mut height: usize = self.len() - 1;

        loop {
            height -= 1;
            if self[height].iter().any(|&x| x) {
                return height;
            }
        }
    }
}

fn drop_rocks(jets: &mut JetPattern, rocks: usize) -> Vec<Vec<bool>> {
    // Init map
    let mut map: Vec<Vec<bool>> = vec![
        vec![true; MAP_WIDTH as usize],
        vec![false; MAP_WIDTH as usize],
        vec![false; MAP_WIDTH as usize],
        vec![false; MAP_WIDTH as usize]
    ];

    for i in (0..rocks).progress() {
        drop_single_rock(jets, &mut map, &i)
        // map.print_map();
    }
    return map;
}

fn drop_single_rock(jets: &mut JetPattern, map: &mut Vec<Vec<bool>>, i: &usize) {
    let rock: Rock = match i % 5 {
        0 => Rock::wide_rock(),
        1 => Rock::plus_rock(),
        2 => Rock::l_rock(),
        3 => Rock::tall_rock(),
        4 => Rock::square_rock(),
        _ => unreachable!()
    };
    let mut position = map.add_rows(&rock.height);

    loop {
        let direction = jets.next_direction();

        if direction == Direction::Right {
            if !rock.collides_right(&map, &position) {
                position.x += 1;
            }
        } else {
            if !rock.collides_left(&map, &position) {
                position.x -= 1;
            }
        }

        if rock.bottoms_out(&map, &position) {
            break;
        }
        position.y -= 1;
    }
    map.add_rock(&rock, &position);
}

fn part_2(input: &str) {
    !todo!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let mut jets = JetPattern::new(&input);
    let rock_map = drop_rocks(&mut jets, 2022);
    println!("Tower height part 1: {}", rock_map.pile_height());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        let mut jets = JetPattern::new(&">><");
        assert_eq!(jets.next_direction(), Direction::Right);
        assert_eq!(jets.next_direction(), Direction::Right);
        assert_eq!(jets.next_direction(), Direction::Left);
        assert_eq!(jets.next_direction(), Direction::Right);
    }

    #[test]
    fn test_wide_rock_bottoms_out_true() {
        let rock = Rock::wide_rock();
        let map = vec![
            vec![false, false, true, false, false],
            vec![false, false, false, false, false]];
        let position = Coordinate {x: 0, y: 1};
        assert_eq!(rock.bottoms_out(&map, &position), true);
    }

    #[test]
    fn test_wide_rock_bottoms_out_false() {
        let rock = Rock::wide_rock();
        let map = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false]];
        let position = Coordinate {x: 0, y: 1};
        assert_eq!(rock.bottoms_out(&map, &position), false);
    }
}
