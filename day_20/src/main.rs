use std::env;

use utils::read_input;

fn parse_numbers(input: &str) -> Vec<i32> {
    let numbers = input
        .lines()
        .map(|c| c.parse::<i32>().expect("Unable to parse {c}"))
        .collect();
    return numbers;
}

#[allow(dead_code)]
fn print_indexes(numbers: &Vec<i32>, indexes: &Vec<usize>) {
    let mut temp: Vec<&i32> = vec![];
    for index in indexes {
        temp.push(&numbers[*index]);
    }
    println!("{:?}", temp);
}

fn circular_position(lenght: i32, current_position: i32, movement: i32) -> usize {
    let new_position = current_position + movement;
    let mut circulation: i32 = new_position / lenght;
    if movement.is_negative() {
        circulation = -1;
    }
    return (new_position + circulation).rem_euclid(lenght) as usize;
}

fn part_1(numbers: &Vec<i32>) -> i32 {
    let mut indexes: Vec<usize> = (0..numbers.len()).collect();
    let lenght: i32 = numbers.len() as i32;
    let mut zero_index: Option<usize> = None;

    for (index, movement) in numbers.iter().enumerate() {
        if movement == &0 {
            zero_index = Some(index);
            continue;
        }
        let current_position = indexes.iter().position(|&i| i==index).unwrap();
        indexes.remove(current_position);
        let new_position = circular_position(lenght, current_position as i32, *movement);
        indexes.insert(new_position, index);
        // print_indexes(numbers, &indexes);
    }
    let starting_point = indexes.iter().position(|&i| i==zero_index.unwrap()).unwrap();
    let mut result: i32 = 0;

    for nth in [1000, 2000, 3000] {
        let index_index = (starting_point + nth).rem_euclid(lenght as usize);
        let number_index = indexes[index_index];
        let to_add = numbers[number_index];
        result += to_add;
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
