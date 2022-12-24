use std::env;

use log::debug;

use utils::{read_input, set_logging_level};

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Vec<String>,
    divisible: usize,
    targets: Vec<usize>,
    reduce_by: usize,
    inscount: usize
}

impl Monkey {
    pub fn from_string(input: &str, reduce_by: usize) -> Monkey {
        let lines: Vec<&str> = input.lines().collect();
        // Parse items
        let items_str: &str = lines[1].split(":").last().unwrap();
        let mut items: Vec<usize> = Vec::new();
        for item in items_str.split(",") {
            items.push(item.trim().parse::<usize>().unwrap())
        }
        // Parse operation
        let mut operation: Vec<String> = Vec::new();
        let operation_str: &str = lines[2].split("= ").last().unwrap();
        for part in operation_str.split(" ") {
            operation.push(part.trim().to_string())
        }
        // Parse testing value
        let divisible: usize = lines[3].split(" ").last().unwrap().trim().parse().unwrap();
        // Item targets
        let mut targets: Vec<usize> = Vec::new();
        for line in lines[4..].iter() {
            targets.push(
                line.split(" ").last().unwrap().trim().parse::<usize>().unwrap()
            )
        }

        Monkey {
            items: items,
            operation: operation,
            divisible: divisible,
            targets: targets,
            reduce_by: reduce_by,
            inscount: 0
        }
    }

    fn test(&mut self, value: &usize) -> bool {
        self.inscount += 1;
        value % self.divisible == 0
    }

    pub fn item_target(&mut self, item: &usize) -> usize {
        match self.test(item) {
            true => return self.targets[0],
            false => return self.targets[1],
        }
    }

    pub fn pass(&mut self, modulo_value: usize) -> Option<usize> {
        let item_option = self.items.pop();
        match item_option {
            Some(item) => {
                // Reduce worry level by calculating modulo
                let new_item = operate(&item, &self.operation).rem_euclid(modulo_value);
                return Some(new_item / self.reduce_by)
            },
            None => return None
        }
    }

    pub fn receive(&mut self, item: usize) {
        self.items.push(item);
    }

}

#[derive(Debug)]
struct Jungle {
    monkeys: Vec<Monkey>
}

impl Jungle {
    pub fn new() -> Jungle {
        Jungle { monkeys: Vec::new() }
    }

    pub fn monkey_from_string(&mut self, input:&str, reduce_by: usize) {
        self.monkeys.push(Monkey::from_string(input, reduce_by))
    }

    fn calculate_modulo_value(&self) -> usize {
        let mut modulo_value: usize = 1;
        for monkey in self.monkeys.iter() {
            modulo_value *= monkey.divisible;
        }
        return modulo_value;
    }

    fn single_round(&mut self) {
        let modulo_value: usize = self.calculate_modulo_value();
        for monkey_index in 0..self.monkeys.len() {
            while let Some(item) = self.monkeys[monkey_index].pass(modulo_value) {
                let target_monkey = self.monkeys[monkey_index].item_target(&item);
                self.monkeys[target_monkey].receive(item)
            }
        }
    }

    pub fn process_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.single_round();
        }
    }
    pub fn report_inspections(&self) {
        let mut inspections: Vec<usize> = Vec::new();
        for (i, monkey) in self.monkeys.iter().enumerate() {
            debug!("Monkey {} inspected items {} times.", i, monkey.inscount);
            inspections.push(monkey.inscount);
        }
        inspections.sort();
        inspections.reverse();
        let monkey_business = inspections[0] * inspections[1];
        println!("Money_business: {}", monkey_business);
    }
}

fn operate(old: &usize, operation: &Vec<String>) -> usize {
    let oper_func = match &operation[1][..] {
        "+" => usize::checked_add,
        "*" => usize::checked_mul,
        &_ => panic!("Unknown operation")
    };

    let b: usize = match &operation[2][..] {
        "old" => *old,
        &_ => operation[2].parse::<usize>().unwrap()
    };

    oper_func(*old, b).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let rounds: usize;
    let reduce_by: usize;
    if args.contains(&String::from("--part2")) {
        rounds= 10000;
        reduce_by = 1;
    } else {
        rounds = 20;
        reduce_by = 3;
    }

    let input = read_input(&args);

    let mut jungle = Jungle::new();
    // Expect Windows file endings
    for monkey_block in input.split("\r\n\r\n") {
        jungle.monkey_from_string(monkey_block, reduce_by);
    }
    debug!("{:?}", jungle);
    jungle.process_rounds(rounds);
    jungle.report_inspections();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operate_plus_old() {
        let operation = vec![
            "old".to_string(),
            "+".to_string(),
            "old".to_string()
        ];
        assert_eq!(operate(&3, &operation), 6);
    }

    #[test]
    fn test_operate_times_4() {
        let operation = vec![
            "old".to_string(),
            "*".to_string(),
            "4".to_string()
        ];
        assert_eq!(operate(&3, &operation), 12);
    }
}