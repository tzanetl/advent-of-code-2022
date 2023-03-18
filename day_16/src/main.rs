use std::str::FromStr;
use std::env;
use std::collections::{HashMap, VecDeque, HashSet};
use std::time::Instant;

use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;

use utils::read_input;

#[derive(Debug, PartialEq, Eq)]
struct InputLine {
    id:String,
    rate: u32,
    leads_to: Vec<String>
}

#[derive(Debug)]
struct ParseInputLineError;

impl FromStr for InputLine {
    type Err = ParseInputLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let id: String = s[6..8].to_string();
        lazy_static! {
            static ref RE_FLOW_RATE: Regex = Regex::new(r"flow rate=(\d+)").unwrap();
            static ref RE_LEADS_TO: Regex =
                Regex::new(r"tunnels? leads? to valves? ([A-Z]+(?:, [A-Z]+)*)").unwrap();
        }
        let rate = RE_FLOW_RATE
            .captures(s)
            .ok_or(ParseInputLineError)?
            .get(1)
            .ok_or(ParseInputLineError)?
            .as_str()
            .parse::<u32>()
            .expect("Cannot parse flow rate");


        let leads_to: Vec<String> = RE_LEADS_TO
            .captures(s)
            .ok_or(ParseInputLineError)?
            .get(1)
            .ok_or(ParseInputLineError)?
            .as_str()
            .replace(" ", "")
            .split(",")
            .map(|x| x.to_string())
            .collect();

        Ok(InputLine { id, rate, leads_to })
    }
}


struct TunnelState <'a>{
    valve: &'a usize,
    visited: HashSet<&'a usize>,
    released_pressure: u32,
    time_remaining: u32
}

//https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
fn make_distance_map(parsed: &Vec<InputLine>) -> Vec<Vec<u32>> {
    let v: usize = parsed.len();
    let id_to_int: Vec<&str> = parsed.into_iter().map(|l| &l.id[..]).collect();

    // Initialize map, rows as "from valve", columns as "to valve"
    let mut distance: Vec<Vec<u32>> = vec![vec![u32::MAX; v]; v];
    // Set diagonals as zero
    for i in 0..v {
        distance[i][i] = 0;
    }
    // Set edges to 1
    for (u, line) in parsed.iter().enumerate() {
        for connection in &line.leads_to {
            let v: usize = id_to_int.iter().position(|e| e == &&connection[..]).unwrap();
            distance[u][v] = 1;
        }
    }
    // Loop combinations
    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                if distance[i][j]
                    > distance[i][k].checked_add(distance[k][j]).or(Some(u32::MAX)).unwrap()
                {
                    distance[i][j] = distance[i][k] + distance[k][j]
                }
            }
        }
    }
    return distance;
}

fn make_valves(parsed: &Vec<InputLine>) -> (HashMap<usize, u32>, Vec<usize>, usize) {
    let mut flow_rates: HashMap<usize, u32> = HashMap::new();
    let mut functioning_valves: Vec<usize> = vec![];
    let mut start_position: Option<usize> = None;

    for (i, p) in parsed.iter().enumerate() {
        if p.id == "AA" {
            start_position = Some(i);
        }
        if p.rate == 0 {
            continue;
        }

        flow_rates.insert(i, p.rate);
        functioning_valves.push(i);
    }

    return (flow_rates, functioning_valves, start_position.expect("Starting position not found"))
}

// Using Breath-First Search
// Ref: https://www.reddit.com/r/adventofcode/comments/zo21au/2022_day_16_approaches_and_pitfalls_discussion/
fn part1(
    flow_rate: &HashMap<usize, u32>,
    valves: &Vec<usize>,
    distance: &Vec<Vec<u32>>,
    start_position: &usize,
    start_time: u32
) -> u32 {
    let mut max_pressure_released: u32 = 0;
    let mut queue: VecDeque<TunnelState> = VecDeque::from(
        [TunnelState {
            valve: start_position,
            visited: HashSet::new(),
            released_pressure: 0,
            time_remaining: start_time
        }]
    );

    while !queue.is_empty() {
        let current_state = queue.pop_front().unwrap();

        for next_valve in valves {
            if current_state.visited.contains(next_valve) {
                continue;
            }
            let travel_time = distance[*current_state.valve][*next_valve];
            if !(travel_time < current_state.time_remaining) {
                continue;
            }
            let mut cache_key: HashSet<&usize> = current_state.visited.clone();
            cache_key.insert(next_valve);
            let time_remaining: u32 = current_state.time_remaining - travel_time - 1;
            let released_pressure: u32 =
                flow_rate[next_valve]
                * time_remaining
                + current_state.released_pressure;
            max_pressure_released = max_pressure_released.max(released_pressure);
            queue.push_back(TunnelState {
                valve: next_valve,
                visited: cache_key,
                released_pressure: released_pressure,
                time_remaining: time_remaining
            })
        }

    }
    return max_pressure_released;
}

fn dereference_vec<T>(hs: Vec<&T>) -> Vec<T> where T: Eq, T: Copy {
    let mut new: Vec<T> = Vec::new();
    for item in hs {
        new.push(*item);
    };
    new
}


fn part2(
    flow_rate: &HashMap<usize, u32>,
    valves: &Vec<usize>,
    distance: &Vec<Vec<u32>>,
    start_position: &usize
) -> u32 {
    let mut max_released_pressure: u32 = 0;
    for r in 2..=14 {
        for myself in valves.iter().combinations(r) {
            let mut elephant = valves.clone();
            elephant.retain(|v| !myself.contains(&v));
            let released_pressure =
                part1(flow_rate, &dereference_vec(myself), distance, start_position, 26)
                + part1(flow_rate, &elephant, distance, start_position, 26);
            max_released_pressure = max_released_pressure.max(released_pressure);
        }
    }
    return max_released_pressure;
}


fn main() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let input: String = read_input(&args);

    let parsed: Vec<InputLine> = input
        .lines()
        .map(
            |line| InputLine::from_str(line)
            .expect(&format!("Unable to parse line: {}", line)[..])
        )
        .collect();

    let distance = make_distance_map(&parsed);

    let (flow_rate, functional_vales, start_position) = make_valves(&parsed);
    let parsing_time = now.elapsed();
    println!("Parsing took: {:.2?}", parsing_time);
    let now = Instant::now();
    let part_1_result = part1(&flow_rate, &functional_vales, &distance, &start_position, 30);
    let part_1_time = now.elapsed();
    println!("Part 1: {part_1_result}");
    println!("Part 1 took: {:.2?}", part_1_time);
    let now = Instant::now();
    let part_2_result = part2(&flow_rate, &functional_vales, &distance, &start_position);
    let part_2_time = now.elapsed();
    println!("Part 2: {part_2_result}");
    println!("Part 2 took: {:.2?}", part_2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_line_from_string_mult() {
        let il = InputLine::from_str(
            r"Valve BB has flow rate=13; tunnels leads to valves CC, AA"
        ).unwrap();
        assert_eq!(
            il,
            InputLine {
                id: "BB".to_string(),
                rate: 13,
                leads_to: vec!["CC".to_string(), "AA".to_string()]
            }
        )
    }

    #[test]
    fn test_input_line_from_string_single() {
        let il = InputLine::from_str(
            r"Valve BB has flow rate=13; tunnel lead to valve CC"
        ).unwrap();
        assert_eq!(
            il,
            InputLine { 
                id: "BB".to_string(),
                rate: 13,
                leads_to: vec!["CC".to_string()]
            }
        )
    }
}
