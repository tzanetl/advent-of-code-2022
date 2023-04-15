use std::env;
use std::collections::VecDeque;

use utils::read_input;

fn parse_numbers(input: &str) -> Vec<i64> {
    let numbers = input
        .lines()
        .map(|c| c.parse::<i64>().expect("Unable to parse {c}"))
        .collect();
    return numbers;
}

#[allow(dead_code)]
fn print_ring(ring: &VecDeque<Pair>) {
    let mut temp: Vec<&i64> = vec![];
    for item in ring {
        temp.push(item.1);
    }
    println!("{:?}", temp);
}


type Pair <'a> = (usize, &'a i64);

fn part_1(numbers: &Vec<i64>) -> i64 {
    let mut ring: VecDeque<Pair> = numbers.iter().enumerate().map(|(i, n)| (i, n)).collect();
    // print_ring(&ring);

    for index in 0..numbers.len() {
        let ring_index: usize = ring.iter().position(|p| p.0 == index).unwrap();

        if ring[ring_index].1 == &0 {
            // print_ring(&ring);
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
        // print_ring(&ring);
    }

    let zero_index: usize = ring.iter().position(|p| p.1 == &0).unwrap();
    let mut result: i64 = 0;
    // println!("{zero_index}");

    for nth in [1000, 2000, 3000] {
        let index: usize = (zero_index + nth).rem_euclid(numbers.len());
        // println!("Index: {index}");
        // println!("{:?}", ring[index].1);
        result += ring[index].1;
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_input(&args);
    let numbers = parse_numbers(&input[..]);
    let part_1_coordinates = part_1(&numbers);
    println!("Part 1 coordinates: {part_1_coordinates}");
}
