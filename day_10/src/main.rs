use std::env;

// use log::debug;

use utils::{read_input, set_logging_level};

struct CPU {
    cycle: usize,
    x_reg: isize,
    report_on: Vec<usize>,
    signal_strength: Vec<isize>
}

impl CPU {
    pub fn new(report_on: Vec<usize>) -> CPU {
        CPU { cycle: 0, x_reg: 1, report_on: report_on, signal_strength: Vec::new() }
    }

    fn tick(&mut self) {
        self.cycle += 1;
        if self.report_on.contains(&self.cycle) {
            self.calculate_strength()
        }
    }

    fn calculate_strength(&mut self) {
        self.signal_strength.push((self.cycle as isize) * self.x_reg);
    }

    fn noop(&mut self) {
        self.tick();
    }

    fn addx(&mut self, value: isize) {
        self.tick();
        self.tick();
        self.x_reg += value;
    }

    pub fn process_line(&mut self, line: &str) {
        let split: Vec<&str> = line.split(" ").collect();
        match split[0] {
            "noop" => self.noop(),
            "addx" => {
                let value: isize = split[1].parse().unwrap();
                self.addx(value);
            },
            &_ => panic!("Unknown command: {}", split[0])
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    set_logging_level(&args);
    let input = read_input(&args);

    let mut cpu = CPU::new(vec![20, 60, 100, 140, 180, 220]);

    for line in input.lines() {
        cpu.process_line(line);
    }
    let sum_signal_strength: isize = cpu.signal_strength.iter().sum();
    println!("Sum signal strenghts: {}", sum_signal_strength)
}
