use std::env;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use log::debug;

use utils::{read_input, set_logging_level};

static ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

#[derive(Debug)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>
}

impl Rucksack {
    pub fn from_string(input: &str) -> Self {
        let lenght: usize = input.len();
        let midpoint: usize = match lenght % 2 {
            0 => lenght / 2,
            _ => panic!("Compartments not evenly spaced")
        };
        debug!("lenght: {lenght}, midpoint: {midpoint}");
        Rucksack {
            first: input[..midpoint].chars().collect(),
            second: input[midpoint..].chars().collect()
        }
    }

    pub fn common_items(&mut self) -> HashSet<&char> {
        self.first.intersection(&self.second).collect()
    }

    pub fn priority_sum(&mut self, alphabet_index: &HashMap<char, i32>) -> i32 {
        let common_items = self.common_items();
        debug!("common: {:?}", common_items);
        let mut sum: i32 = 0;

        for c in common_items {
            sum += alphabet_index[c];
        }

        debug!("priority: {sum}");
        debug!("---------------");
        return sum
    }

}

fn generate_alphabet_index() -> HashMap<char, i32> {
    let mut index: i32 = 1;
    let mut alphabet_index: HashMap<char, i32> = HashMap::new();

    for c in ALPHABET {
        alphabet_index.insert(c, index);
        index += 1;
    }
    return alphabet_index
}

fn dereference_hashmap<T>(hs: HashSet<&T>) -> HashSet<T> where T: Eq, T: Hash, T: Copy {
    let mut new: HashSet<T> = HashSet::new();
    for item in hs {
        new.insert(*item);
    };
    new
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let alphabet_index = generate_alphabet_index();

    let mut rucksack: Rucksack;
    let mut priority_sum: i32 = 0;

    for line in input.lines() {
        rucksack = Rucksack::from_string(&line);
        priority_sum += rucksack.priority_sum(&alphabet_index)
    }
    println!("Total priority: {priority_sum}");

    // Part 2
    priority_sum = 0;
    let mut input_lines = input.lines();
    let mut first_elf: &str = input_lines.next().unwrap();
    let mut second_elf: &str;
    let mut third_elf: &str;

    loop {
        second_elf = input_lines.next().unwrap();
        rucksack = Rucksack{
            first: first_elf.chars().collect(),
            second: second_elf.chars().collect()
        };
        // Make new rucksack based on 3rd elves items and common items from 1st and 2nd elves
        let common_items = rucksack.common_items();
        third_elf = input_lines.next().unwrap();
        rucksack = Rucksack{
            first: third_elf.chars().collect(),
            second: dereference_hashmap(common_items)
        };
        priority_sum += rucksack.priority_sum(&alphabet_index);

        // Are all lines processed?
        let roll_over = input_lines.next();
        if roll_over == None {
            break;
        }
        first_elf = roll_over.unwrap();
    }
    println!("Total priority per 3 elves: {priority_sum}");
}
