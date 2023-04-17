use std::env;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use log::debug;
use itertools::Itertools;

use utils::{read_input, set_logging_level};

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

struct ParseOperatorError;

impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            &_ => Err(ParseOperatorError)
        }
    }
}

impl Operator {
    fn operate(&self, left: &isize, right: &isize) -> isize {
        match self {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Branch {
    left: String,
    right: String,
    operator: Operator
}

#[derive(Debug)]
struct ParseBranchError;

impl FromStr for Branch {
    type Err = ParseBranchError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, operator_str, right) = s
            .trim()
            .splitn(3, &" ")
            .collect_tuple()
            .ok_or(ParseBranchError)?;
        let operator = Operator::from_str(operator_str).map_err(|_| ParseBranchError)?;
        Ok(Self {left: left.to_string(), right: right.to_string(), operator})
    }
}

fn parse_input(input: &str) -> (HashMap<String, Branch>, HashMap<String, isize>) {
    let mut branches: HashMap<String, Branch> = HashMap::new();
    let mut values: HashMap<String, isize> = HashMap::new();

    for line in input.lines() {
        let (key, message) = line.split_once(":").unwrap();
        let parsed_message = message.trim().parse::<isize>();

        match parsed_message {
            Ok(v) => {
                values.insert(key.to_string(), v);
            },
            Err(_) => {
                debug!("{message}");
                branches.insert(key.to_string(), Branch::from_str(message).unwrap());
            },
        }
    }
    return (branches, values);
}

fn part_1(branches: HashMap<String, Branch>, mut values: HashMap<String, isize>)
    -> HashMap<String, isize>
{
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("root");

    while !queue.is_empty() {
        if values.contains_key(queue[0]) {
            queue.pop_front();
            continue;
        }

        let branch = &branches[queue[0]];
        let left_value: Option<&isize> = values.get(&branch.left);
        let right_value: Option<&isize> = values.get(&branch.right);

        if left_value == None {
            queue.push_front(&branch.left)
        }

        if right_value == None {
            queue.push_front(&branch.right)
        }

        if let (Some(left), Some(right)) = (left_value, right_value) {
            let value: isize = branch.operator.operate(left, right);
            let key = queue.pop_front().unwrap();
            values.insert(key.to_string(), value);
        }
    }
    return values;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_input(&args);
    set_logging_level(&args);
    let (branches, values) = parse_input(&input);
    let all_values = part_1(branches, values);
    println!("Root will yell: {:?}", all_values["root"]);
}
