use std::{env, str::FromStr, collections::HashMap};

use utils;

#[derive(Debug, Eq, PartialEq, Hash)]
enum RPSSelect {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPSSelect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPSSelect::Rock),
            "B" | "Y" => Ok(RPSSelect::Paper),
            "C" | "Z" => Ok(RPSSelect::Scissors),
            _ => Err(()),
        }
    }
}

impl RPSSelect {
    pub fn from_result(opponent: &RPSSelect, result: &str) -> Self {
        match (opponent, result) {
            (&RPSSelect::Rock, "X") => RPSSelect::Scissors,
            (&RPSSelect::Rock, "Y") => RPSSelect::Rock,
            (&RPSSelect::Rock, "Z") => RPSSelect::Paper,
            (&RPSSelect::Paper, "X") => RPSSelect::Rock,
            (&RPSSelect::Paper, "Y") => RPSSelect::Paper,
            (&RPSSelect::Paper, "Z") => RPSSelect::Scissors,
            (&RPSSelect::Scissors, "X") => RPSSelect::Paper,
            (&RPSSelect::Scissors, "Y") => RPSSelect::Scissors,
            (&RPSSelect::Scissors, "Z") => RPSSelect::Rock,
            _ => panic!("Invalid result")
        }
    }

    pub fn from_opponent(s: &str) -> Self {
        match s {
            "A" => RPSSelect::Rock,
            "B" => RPSSelect::Paper,
            "C" => RPSSelect::Scissors,
            _ => panic!("Unknown RPS selection")
        }
    }

    pub fn points(&mut self) -> i32 {
        match self {
            RPSSelect::Rock => 1,
            RPSSelect::Paper => 2,
            RPSSelect::Scissors => 3
        }
    }
}

#[derive(Debug)]
struct RPSMatch {
    me: RPSSelect,
    opponent: RPSSelect
}

impl FromStr for RPSMatch {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        match split.len() {
            2 => Ok(
                RPSMatch {
                    me: RPSSelect::from_str(&split[1])?,
                    opponent: RPSSelect::from_str(&split[0])?
                },
            ),
            _ => Err(())
        }
    }
}

impl RPSMatch {
    pub fn from_match_result(s: &str) -> Self {
        let split: Vec<&str> = s.split(" ").collect();
        let opponent = RPSSelect::from_opponent(split[0]);
        let me = RPSSelect::from_result(&opponent, split[1]);
        RPSMatch { me: me, opponent: opponent }
    }

    pub fn match_points(&mut self) -> i32 {
        let rps_map:HashMap<(&RPSSelect, &RPSSelect), i32> = HashMap::from([
            ((&RPSSelect::Rock, &RPSSelect::Rock), 3),
            ((&RPSSelect::Rock, &RPSSelect::Paper), 0),
            ((&RPSSelect::Rock, &RPSSelect::Scissors), 6),
            ((&RPSSelect::Paper, &RPSSelect::Rock), 6),
            ((&RPSSelect::Paper, &RPSSelect::Paper), 3),
            ((&RPSSelect::Paper, &RPSSelect::Scissors), 0),
            ((&RPSSelect::Scissors, &RPSSelect::Rock), 0),
            ((&RPSSelect::Scissors, &RPSSelect::Paper), 6),
            ((&RPSSelect::Scissors, &RPSSelect::Scissors), 3)
        ]);
        let mut points: i32 = rps_map[&(&self.me, &self.opponent)];
        points += self.me.points();
        return points;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = utils::read_input(&args);

    let mut rps_match: RPSMatch;
    let mut my_points: i32 = 0;

    for line in input.lines() {
        rps_match = RPSMatch::from_str(&line).unwrap();
        my_points += rps_match.match_points();
    }
    println!("My points: {}", my_points);

    let mut my_points_correct: i32 = 0;
    for line in input.lines() {
        rps_match = RPSMatch::from_match_result(&line);
        my_points_correct += rps_match.match_points();
    }
    println!("My points after correction: {}", my_points_correct);
}
