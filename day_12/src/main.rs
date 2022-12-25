use std::env;
use std::collections::HashSet;

use log::debug;

use utils::{read_input, set_logging_level};

// Random char added to beginning so a == 1
static ALPHABET: &'static [char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l','m',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cell {
    value: usize,
    reached: bool
}


#[derive(Debug)]
struct Mountain {
    grid: Vec<Cell>,
    width: usize,
    start: usize,
    finish: usize
}

impl Mountain {
    // fn icell(&self, row: &usize, col: &usize) -> &Cell {
    //     // https://stackoverflow.com/a/2151141/14536215
    //     // https://en.wikipedia.org/wiki/Row-_and_column-major_order
    //     &self.grid[(self.width * row + col)]
    // }

    pub fn from_input(input: &str) -> Mountain {
        let mut width: usize = 0;
        let mut grid: Vec<Cell> = Vec::new();
        let mut start: usize = 0;
        let mut finish: usize = 0;

        for line in input.lines() {
            width = line.len();
            for c in line.chars() {
                let cell = match c {
                    'S' => {
                        start = grid.len();
                        Cell {
                            value: ALPHABET.iter().position(|&x| x == 'a').unwrap(),
                            reached: true
                        }
                    },
                    'E' => {
                        finish = grid.len();
                        Cell {
                            value: ALPHABET.iter().position(|&x| x == 'z').unwrap(),
                            reached: false
                        }
                    },
                    char => {
                        Cell {
                            value: ALPHABET.iter().position(|&x| x == char).unwrap(),
                            reached: false
                        }
                    }
                };
                grid.push(cell)
            }
        }
        Mountain { grid: grid, width: width, start: start, finish: finish }
    }

    /// Parse surrounding unreached grid cell indexes
    fn surrounding(&self, index: usize) -> HashSet<usize> {
        let width_i = self.width as isize;
        let index_i = index as isize;
        let mut reachable: HashSet<usize> = HashSet::new();
        let shift: Vec<isize> = vec![
            1,
            width_i,
            -1,
            -width_i
        ];

        for value in shift {
            let new_index: usize = match usize::try_from(index_i + value) {
                Ok(value) => value,
                Err(_) => continue
            };
            if new_index >= self.grid.len() {
                continue;
            }
            if self.grid[new_index].reached {
                continue;
            }
            reachable.insert(new_index);
        }

        return reachable
    }

    pub fn travel_to_destination(&mut self) -> Option<usize> {
        let mut steps: usize = 0;
        let mut starting_cells: HashSet<usize> = HashSet::from([self.start]);

        loop {
            steps += 1;
            let mut reached: HashSet<usize> = HashSet::new();
            for starting_cell_id in starting_cells {
                let max_value = self.grid[starting_cell_id].value + 1;
                for id_to_check in self.surrounding(starting_cell_id) {
                    let cell = &mut self.grid[id_to_check];

                    if cell.value > max_value {
                        continue;
                    }
                    if id_to_check == self.finish {
                        cell.reached = true;
                        return Some(steps);
                    }
                    cell.reached = true;
                    reached.insert(id_to_check);
                }
            }
            if reached.len() == 0 {
                return None;
            }
            starting_cells = reached;
        }
    }

    pub fn reset_grid(&mut self) {
        for cell in self.grid.iter_mut() {
            cell.reached = false;
        }
        self.grid[self.start].reached = true;
    }

    pub fn starting_points(&self) -> HashSet<usize> {
        let starting_value = ALPHABET.iter().position(|&x| x == 'a').unwrap();
        let mut starting_points = HashSet::new();

        for (index, cell) in self.grid.iter().enumerate() {
            if cell.value == starting_value {
                starting_points.insert(index);
            }
        }
        debug!("Starting points: {:?}", starting_points);
        return starting_points;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    let mut mountain = Mountain::from_input(&input);
    debug!("{:?}", mountain);
    let steps: usize = mountain.travel_to_destination().expect("Destination not reachable");
    println!("Destination reached in {} steps", steps);

    let mut minimum_steps: usize = usize::MAX;

    for starting_point in mountain.starting_points() {
        mountain.start = starting_point;
        mountain.reset_grid();
        let steps = mountain.travel_to_destination();
        match steps {
            Some(value) => {
                debug!("{}", value);
                if value < minimum_steps {
                    minimum_steps = value;
                }
            },
            None => debug!("Destination not reached when starting from {}", starting_point)
        }
    }
    println!("Destination reached in {} steps from any starting point", minimum_steps);
}
