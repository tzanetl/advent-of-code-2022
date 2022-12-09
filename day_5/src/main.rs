use std::env;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use log::debug;

use utils::{read_input, set_logging_level};

#[derive(Clone, Debug)]
struct ContainerYard {
    piles: HashMap<u8, Vec<char>>
}

impl ContainerYard {
    pub fn from_string(input: &str) -> ContainerYard{
        let mut lines = input.lines();
        // Get last number at the numbers line
        // Expects " 1   2   3 "
        let n_piles: u8 = lines
            .next_back()
            .unwrap()
            .trim()
            .split(" ")
            .into_iter()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        debug!("N piles: {:?}", n_piles);

        let mut container: char;
        let mut piles: HashMap<u8, Vec<char>> = HashMap::new();
        for i in 1..n_piles + 1 {
            piles.insert(i, Vec::<char>::new());
        }
        // let container_row = lines.next_back().unwrap().as_bytes();
        while let Some(container_row) = lines.next_back() {
            for i in 0..n_piles {
                container = container_row.as_bytes()[(1 + i * 4) as usize] as char;
                if container == ' ' {
                    continue;
                }
                debug!("{}: {container}", i + 1);
                piles.entry(i + 1).or_insert(Vec::<char>::new()).push(container);
            }
        }
        debug!("{:?}", piles);
        ContainerYard {piles}
    }

    pub fn move_with_string(&mut self, input: &str) {
        let split: Vec<&str> = input.split(" ").collect();
        let move_n: u8 = split[1].parse().unwrap();
        let move_from: u8 = split[3].parse().unwrap();
        let move_to: u8 = split[5].parse().unwrap();
        debug!("Movement: {input}");
        debug!("Pre :{:?}", self.piles);
        for _ in 0..move_n {
            let container: char = match self.piles.entry(move_from){
                Entry::Occupied(mut v) => v.get_mut().pop().unwrap(),
                Entry::Vacant(_) => panic!("Missing pile {move_from}")
            };
            match self.piles.entry(move_to){
                Entry::Occupied(mut v) => v.get_mut().push(container),
                Entry::Vacant(_) => panic!("Missing pile {move_from}")
            };
        }
        debug!("Post:{:?}", self.piles)
    }

    pub fn move_stack_with_string(&mut self, input: &str) {
        let split: Vec<&str> = input.split(" ").collect();
        let move_n: u8 = split[1].parse().unwrap();
        let move_from: u8 = split[3].parse().unwrap();
        let move_to: u8 = split[5].parse().unwrap();
        debug!("Movement: {input}");
        debug!("Pre :{:?}", self.piles);

        let mut container = match self.piles.entry(move_from){
            Entry::Occupied(mut v) => {
                let temp = v.get_mut();
                temp.split_off(temp.len() - (move_n as usize))
            },
            Entry::Vacant(_) => panic!("Missing pile {move_from}")
        };
        match self.piles.entry(move_to){
            Entry::Occupied(mut v) => v.get_mut().append(&mut container),
            Entry::Vacant(_) => panic!("Missing pile {move_to}")
        };

        debug!("Post:{:?}", self.piles)
    }

    pub fn top_row(&self) -> String{
        let mut top_row = String::new();
        let n_piles: &u8 = self.piles.keys().max().unwrap();
        for i in 1..n_piles + 1 {
            let pile: &Vec<char> = &self.piles[&i];
            if pile.len() == 0 {
                top_row.push('_');
            } else {
                top_row.push(pile[pile.len() - 1]);
            }
        }
        debug!("{:?}", self);
        return top_row;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);
    // Assumes Windows line endings with carriage return
    let split: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut container_yard = ContainerYard::from_string(split[0]);
    let mut container_yard_alt = container_yard.clone();

    for line in split[1].lines() {
        container_yard.move_with_string(line);
    };
    let top_row = container_yard.top_row();
    println!("Top row: {top_row}");

    for line in split[1].lines() {
        container_yard_alt.move_stack_with_string(line);
    };
    let top_row_alt = container_yard_alt.top_row();
    println!("Top row with CrateMover 9001: {top_row_alt}")
}
