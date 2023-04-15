use std::env;
use std::collections::VecDeque;

use log::debug;

use utils::{read_input, set_logging_level};

fn parse_numbers(input: &str) -> Vec<i64> {
    let numbers = input
        .lines()
        .map(|c| c.parse::<i64>().expect("Unable to parse {c}"))
        .collect();
    return numbers;
}

#[allow(dead_code)]
fn print_ring(ring: &VecDeque<Pair>) -> String{
    let mut temp: Vec<&i64> = vec![];
    for item in ring {
        temp.push(&item.1);
    }
    return format!("{:?}", temp);
}

type Pair = (usize, i64);

fn decipher(numbers: &Vec<i64>, rounds: usize, key: i64) -> i64 {
    let mut ring: VecDeque<Pair> = numbers.iter().enumerate().map(|(i, n)| (i, (n * key))).collect();
    debug!("{}", print_ring(&ring));

    for _ in 0..rounds {
        for index in 0..numbers.len() {
            let ring_index: usize = ring.iter().position(|p| p.0 == index).unwrap();

            if ring[ring_index].1 == 0 {
                debug!("{}", print_ring(&ring));
                continue;
            }

            let item: Pair = ring.remove(ring_index).unwrap();
            let rotations: usize = item.1.abs().rem_euclid(ring.len() as i64) as usize;

            if item.1.is_positive() {
                ring.rotate_left(rotations)
            } else {
                ring.rotate_right(rotations)
            }

            ring.insert(ring_index, item);
            debug!("{}", print_ring(&ring));
        }
    }

    let zero_index: usize = ring.iter().position(|p| p.1 == 0).unwrap();
    let mut result: i64 = 0;
    debug!("Zero index: {zero_index}");

    for nth in [1000, 2000, 3000] {
        let index: usize = (zero_index + nth).rem_euclid(numbers.len());
        debug!("Index: {index}");
        debug!("{:?}", ring[index].1);
        result += ring[index].1;
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_input(&args);
    set_logging_level(&args);
    let numbers = parse_numbers(&input[..]);
    let part_1_coordinates = decipher(&numbers, 1, 1);
    println!("Part 1 coordinates: {part_1_coordinates}");
    let part_2_coordinates = decipher(&numbers, 10, 811589153);
    println!("Part 2 coordinates: {part_2_coordinates}");
}
