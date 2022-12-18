use std::env;

// use log::debug;

use utils::{read_input, set_logging_level};

struct CPU {
    cycle: isize,
    x_reg: isize,
    report_on: Vec<isize>,
    signal_strength: Vec<isize>,
    picture: String
}

impl CPU {
    pub fn new(report_on: Vec<isize>) -> CPU {
        CPU {
            cycle: 0,
            x_reg: 1,
            report_on: report_on,
            signal_strength: Vec::new(),
            picture: String::new()
        }
    }

    fn tick(&mut self) {
        if (self.cycle != 0) & (self.cycle % 40 == 0) {
            self.picture.push('\n');
        }
        self.draw();

        self.cycle += 1;

        if self.report_on.contains(&self.cycle) {
            self.calculate_strength();
        }
    }

    fn draw(&mut self) {
        let position: &isize = &self.cycle.rem_euclid(40);
        if ((self.x_reg - 1)..=(self.x_reg + 1)).contains(position) {
            self.picture.push('#');
        } else {
            self.picture.push('.');
        }
    }

    fn calculate_strength(&mut self) {
        self.signal_strength.push(self.cycle * self.x_reg);
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
    println!("Sum signal strenghts: {}", sum_signal_strength);
    println!("{}", cpu.picture);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt_output() {
        let args: Vec<String> = vec!["--test".to_string()];
        set_logging_level(&args);
        let input = read_input(&args);

        let mut cpu = CPU::new(vec![20, 60, 100, 140, 180, 220]);

        for line in input.lines() {
            cpu.process_line(line);
        }
        let correct_picture = "\
        ##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######.....".to_string();

        assert_eq!(cpu.picture, correct_picture);
    }
}
